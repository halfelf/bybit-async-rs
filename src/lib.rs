//#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]
mod config;
mod error;
mod macros;
pub mod model;
mod parser;
pub mod rest;
pub mod websocket;

pub use config::Config;
pub use error::{BinanceError, BinanceResponse, BinanceResponseError};
pub use rest::Binance;
