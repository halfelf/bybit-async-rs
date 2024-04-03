pub mod topics;

use crate::{
    error::BybitError::{self, *},
    models::Product,
    Config,
};
use fehler::{throw, throws};
use futures::{stream::Stream, SinkExt, StreamExt};
use log::debug;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, value::RawValue};
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub trait ParseMessage: Sized {
    fn parse(stream: &str, data: &str) -> Result<Self, BybitError>;
    fn ping() -> Self;
}

pub struct BybitWebsocket<M> {
    stream: WSStream,
    _phantom: PhantomData<M>,
}

impl<M> BybitWebsocket<M>
where
    M: ParseMessage,
{
    #[throws(BybitError)]
    pub async fn new<I, S>(config: Config, topics: I) -> BybitWebsocket<M>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut combined = String::new();
        for topic in topics {
            if !combined.is_empty() {
                combined.push('/');
            }

            combined.push_str(topic.as_ref())
        }

        if combined.is_empty() {
            throw!(EmptyTopics)
        }

        let base = match config.product {
            Product::Spot => &config.spot_ws_endpoint,
            Product::UsdMFutures => &config.usdm_futures_ws_endpoint,
            Product::CoinMFutures => &config.coinm_futures_ws_endpoint,
            Product::EuropeanOptions => &config.european_options_ws_endpoint,
        };
        let endpoint = Url::parse(&format!("{}/stream?streams={}", base, combined)).unwrap();
        debug!("ws endpoint: {endpoint:?}");
        let (stream, _) = match connect_async(endpoint).await {
            Ok(v) => v,
            Err(tungstenite::Error::Http(ref http)) => throw!(StartWebsocketError(
                http.status(),
                String::from_utf8_lossy(http.body().as_deref().unwrap_or_default()).to_string()
            )),
            Err(e) => throw!(e),
        };
        Self {
            stream,
            _phantom: PhantomData,
        }
    }
}

impl<M> BybitWebsocket<M> {
    #[throws(BybitError)]
    pub async fn pong(&mut self) {
        self.stream.send(Message::Pong(vec![])).await?
    }
}

#[derive(Deserialize)]
struct MessageWithTopic<'a> {
    stream: String,
    #[serde(borrow)]
    data: &'a RawValue,
}

impl<M> Stream for BybitWebsocket<M>
where
    M: ParseMessage + Unpin + std::fmt::Debug,
{
    type Item = Result<M, BybitError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let c = match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(c))) => c,
            Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e.into()))),
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
        };
        let msg = match c {
            Message::Text(msg) => msg,
            Message::Ping(..) => return Poll::Ready(Some(Ok(M::ping()))),
            Message::Binary(_) | Message::Frame(_) | Message::Pong(..) => return Poll::Pending,
            Message::Close(_) => return Poll::Ready(None),
        };

        let t: MessageWithTopic = match from_str(&msg) {
            Ok(v) => v,
            Err(e) => return Poll::Ready(Some(Err(e.into()))),
        };

        Poll::Ready(Some(M::parse(&t.stream, t.data.get())))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
