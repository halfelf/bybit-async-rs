mod coinm;
mod models;
mod spot;
mod usdm;

pub use self::{
    coinm::CoinMWebsocketMessage, models::*, spot::SpotWebsocketMessage, usdm::UsdMWebsocketMessage,
};
use crate::model::Product;
use anyhow::Error;
use futures::{stream::Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, value::RawValue};
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
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

impl<M> BinanceWebsocket<M> {
    pub fn new(stream: WSStream) -> Self {
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
