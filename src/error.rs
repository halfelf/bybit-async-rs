use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug, Clone, Error)]
#[error("Binance returns error: {msg}")]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceResponseError),
}

impl<T: for<'a> Deserialize<'a>> BinanceResponse<T> {
    pub fn to_result(self) -> Result<T, BinanceResponseError> {
        match self {
            BinanceResponse::Success(t) => Ok(t),
            BinanceResponse::Error(e) => Err(e),
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum BinanceError {
    #[error("No Api key set for private api")]
    MissingApiKey,
    #[error("No Api secret set for private api")]
    MissingApiSecret,
    #[error("No stream is subscribed")]
    NoStreamSubscribed,
    #[error("Websocket is closed")]
    WebsocketClosed,
    #[error("Topics is empty")]
    EmptyTopics,
    #[error("Unknown stream {0}")]
    UnknownStream(String),
    #[error("Stream {0} not implemented yet")]
    StreamNotImplemented(String),
    #[error("Error when try to connect websocket: {0} - {1}")]
    StartWebsocketError(StatusCode, String),
}
