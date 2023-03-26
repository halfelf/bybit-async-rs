mod coinm;
mod models;
mod spot;
mod usdm;

pub use self::{
    coinm::CoinMWebsocketMessage, models::*, spot::SpotWebsocketMessage, usdm::UsdMWebsocketMessage,
};
use crate::{error::BinanceError::*, model::Product, Config};
use anyhow::Error;
use fehler::{throw, throws};
use futures::{stream::Stream, StreamExt};
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

pub trait WebsocketMessage: Sized {
    const PRODUCT: Product;

    fn parse(stream: &str, data: &str) -> Result<Self, Error>;
}

pub struct BinanceWebsocket<M> {
    stream: WSStream,
    _phantom: PhantomData<M>,
}

impl<M> BinanceWebsocket<M>
where
    M: WebsocketMessage,
{
    #[throws(Error)]
    pub async fn new<I, S>(topics: I) -> BinanceWebsocket<M>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let config = Config::default();
        Self::with_config(&config, topics).await?
    }

    #[throws(Error)]
    pub async fn with_config<I, S>(config: &Config, topics: I) -> BinanceWebsocket<M>
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

        let base = match M::PRODUCT {
            Product::Spot => &config.ws_endpoint,
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

#[derive(Deserialize)]
struct MessageWithTopic<'a> {
    stream: String,
    #[serde(borrow)]
    data: &'a RawValue,
}

impl<M> Stream for BinanceWebsocket<M>
where
    M: WebsocketMessage + Unpin + std::fmt::Debug,
{
    type Item = Result<M, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let c = match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(c))) => c,
            Poll::Ready(Some(Err(e))) => return dbg!(Poll::Ready(Some(Err(e.into())))),
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
        };
        let msg = match c {
            Message::Text(msg) => msg,
            Message::Binary(_) | Message::Frame(_) | Message::Pong(..) | Message::Ping(..) => {
                return Poll::Pending
            }
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