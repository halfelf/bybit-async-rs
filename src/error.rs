use reqwest::header::InvalidHeaderValue;
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct BybitResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Debug, Error)]
pub enum BybitError {
    #[error("No Api key set for private api")]
    MissingApiKey,
    #[error("No Api secret set for private api")]
    MissingApiSecret,
    #[error("Websocket is closed")]
    WebsocketClosed,
    #[error("Topics is empty")]
    EmptyTopics,
    #[error("Unknown stream {0}")]
    UnknownStream(String),
    #[error("Stream {0} not implemented yet")]
    StreamNotImplemented(String),
    #[error("User data stream event {0} not implemented yet")]
    UserDataStreamEventNotImplemented(String),
    #[error("Error when try to connect websocket: {0} - {1}")]
    StartWebsocketError(StatusCode, String),
    #[error("The field for the given event type {0} in user data stream is empty")]
    EmptyUserDataStream(String),
    #[error("Bybit returns error: {code} - {msg}")]
    BybitResponse { code: i64, msg: String },

    #[error(transparent)]
    Websocket(#[from] tungstenite::Error),
    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),
    #[error(transparent)]
    HttpHeader(#[from] InvalidHeaderValue),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl From<BybitResponseError> for BybitError {
    fn from(v: BybitResponseError) -> Self {
        Self::BybitResponse {
            code: v.code,
            msg: v.msg,
        }
    }
}
