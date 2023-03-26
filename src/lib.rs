//#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]
//! Binance Async
//! Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs)
//! with Async/Await and ergonomic design.
//!
//! This repo is at its early stage, not all requests/websockets are implemented.
//! However, the related mechanism is already there, adding support for new requests/websocket events
//! should only require several lines of code. PRs are very welcomed!
//!

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
pub use websocket::BinanceWebsocket;
