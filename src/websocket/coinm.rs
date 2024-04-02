use super::models::AggregateTrade;
use crate::{
    error::BybitError::{self, *},
    models::Product,
    websocket::ParseMessage,
};
use fehler::{throw, throws};
use serde::Serialize;
use serde_json::from_str;

#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum WebsocketMessage {
    Ping,
    AggregateTrade(AggregateTrade),
}

impl ParseMessage for WebsocketMessage {
    const PRODUCT: Product = Product::CoinMFutures;

    #[throws(BybitError)]
    fn parse(stream: &str, data: &str) -> Self {
        if stream.ends_with("@aggTrade") {
            let value: AggregateTrade = from_str(data)?;
            Self::AggregateTrade(value)
        } else if stream.contains("@markPrice") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.starts_with("!markPrice@arr") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.contains("@kline_") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.contains("@continuousKline_") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@miniTicker") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!miniTicker@arr" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@ticker") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!ticker@arr" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@bookTicker") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!bookTicker" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@forceOrder") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!forceOrder@arr" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@depth") || stream.contains("@depth@") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.contains("@depth") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@compositeIndex") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!contractInfo" {
            throw!(StreamNotImplemented(stream.into()))
        } else {
            throw!(UnknownStream(stream.into()))
        }
    }

    fn ping() -> Self {
        Self::Ping
    }
}
