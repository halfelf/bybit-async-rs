pub mod topics;

use crate::{
    error::BybitError::{self, *},
    models::Product,
    Config,
};
use fehler::{throw, throws};
use futures::{stream::Stream, SinkExt, StreamExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, value::RawValue};
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use std::time::SystemTime;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub trait ParseMessage: Sized {
    fn parse(topic: &str, data: &str) -> Result<Self, BybitError>;
    fn parse_succ(succ: &str) -> Result<Self, BybitError>;
    fn ping() -> Self;
}

pub struct BybitWebsocket<M> {
    stream: WSStream,
    _phantom: PhantomData<M>,
    private: bool,
}

impl<M> BybitWebsocket<M>
where
    M: ParseMessage,
{
    #[throws(BybitError)]
    pub async fn new(config: Config) -> BybitWebsocket<M>
    {
        let base = if config.api_key.is_none() {
            match config.product {
                Product::Spot => &config.spot_ws_endpoint,
                Product::UsdMFutures => &config.usdm_futures_ws_endpoint,
                Product::CoinMFutures => &config.coinm_futures_ws_endpoint,
                Product::EuropeanOptions => &config.european_options_ws_endpoint,
            }
        }
        else {
            &config.private_ws_endpoint
        };
        let endpoint = Url::parse(base).unwrap();
        let (mut stream, _) = match connect_async(endpoint).await {
            Ok(v) => v,
            Err(tungstenite::Error::Http(ref http)) => throw!(StartWebsocketError(
                http.status(),
                String::from_utf8_lossy(http.body().as_deref().unwrap_or_default()).to_string()
            )),
            Err(e) => throw!(e),
        };

        let private = config.api_secret.is_some() && config.api_key.is_some();
        if config.api_key.is_some() {
            let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let in_ms = since_epoch.as_millis() as u64;
            let expires = in_ms + 1_000;

            let mut mac = Hmac::<Sha256>::new_from_slice(config.api_secret.unwrap().as_bytes()).unwrap();
            let sign_message = format!("GET/realtime{}", expires);
            mac.update(sign_message.as_bytes());
            let signature = hex::encode(mac.finalize().into_bytes());

            let msg = serde_json::to_string(&serde_json::json!({
                "op": "auth",
                "args": [config.api_key.unwrap(), expires, signature],
            }))?;
            stream.send(Message::Text(msg)).await?;
        }

        Self {
            stream,
            _phantom: PhantomData,
            private,
        }
    }

    pub async fn subscribe(&mut self, topics: Vec<&str>) -> Result<(), BybitError>
    {
        let topics: Vec<&str> = topics.into_iter().collect();
        let msg = serde_json::to_string(&serde_json::json!({
            "op": "subscribe",
            "args": topics,
        }))?;
        self.stream.send(Message::Text(msg)).await?;
        Ok(())
    }
}

impl<M> BybitWebsocket<M> {
    #[throws(BybitError)]
    pub async fn pong(&mut self) {
        self.stream.send(Message::Pong(vec![])).await?
    }
}

#[derive(Deserialize)]
struct PublicMessage<'a> {
    pub topic: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub ts: u64,
    #[serde(borrow)]
    pub data: &'a RawValue,
}

#[derive(Deserialize)]
struct PrivateMessage<'a> {
    pub id: String,
    pub topic: String,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    #[serde(borrow)]
    pub data: &'a RawValue,
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

        println!("{:?}", &msg);

        if self.private {
            let try_message = from_str::<PrivateMessage>(&msg);
            if try_message.is_ok() {
                let message = try_message.unwrap();
                println!("try_message: id: {}, topic: {}, ts: {}, data: {}", &message.id, &message.topic, &message.creation_time, &message.data);
                Poll::Ready(Some(M::parse(&message.topic, message.data.get())))
            }
            else {  // auth/sub success
                println!("check parse succ");
                Poll::Ready(Some(Ok(M::parse_succ(&msg).unwrap())))
            }
        }
        else {
            let message = from_str::<PublicMessage>(&msg)?;
            println!("try_message: topic: {}, type: {}, ts: {}, data: {}", &message.topic, &message.type_, &message.ts, &message.data);
            Poll::Ready(Some(M::parse(&message.topic, message.data.get())))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
