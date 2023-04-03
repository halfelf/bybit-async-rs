//#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]
//! # Binance Async
//! Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs)
//! with Async/Await and ergonomic design.
//!
//! This repo is at its early stage, not all requests/websockets are implemented.
//! However, the related mechanism is already there: adding support for new requests/websocket events
//! should only require several lines of code. PRs are very welcomed!
//!
//! ## Design Goal
//!
//! Besides the async/await support, this library aims for being ergonomic by leveraging types. The struct
//! for REST and Websocket only provides limited functions for you to call: for example, `Binance` is
//! the struct for REST requests and it only exposes one function: `fn request`. Additionally,
//! which API endpoint to call and what parameter to carry are all stored in the type information.
//! for example, creating a new order is
//! ```rust
//! let binance = Binance::new();
//! binance.request(usdm::NewOrderRequest {
//!     symbol: "ethusdt".into(),
//!     r#type: OrderType::Limit,
//!     side: OrderSide::Buy,
//!
//!     price: Decimal::from_f64(1500.),
//!     quantity: Decimal::from_f64(0.004),
//!     time_in_force: Some(TimeInForce::GTC),
//!     ..Default::default()
//! })
//! ```
//!
//! As you can see, `usdm::NewOrderRequest` itself knows which endpoint ("/fapi/v1/order") to send this request to. Moreover,
//! all the request structs have `Default` implemented, which allows you to express different parameter
//! combinations without making the code verbose.
//!
//! ## Examples
//!
//! ### Send a New Order and Cancel it
//!
//! ```rust
//! async fn main() {
//!    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
//!    let resp = binance
//!        .request(usdm::NewOrderRequest {
//!            symbol: "ethusdt".into(),
//!            r#type: OrderType::Limit,
//!            side: OrderSide::Buy,
//!
//!            price: Decimal::from_f64(1500.),
//!            quantity: Decimal::from_f64(0.004),
//!            time_in_force: Some(TimeInForce::GTC),
//!            ..Default::default()
//!        })
//!        .await?;
//!    println!("{resp:?}");
//!
//!    let resp = binance
//!        .request(usdm::CancelOrderRequest {
//!            symbol: "ethusdt".into(),
//!            order_id: Some(resp.order_id),
//!            ..Default::default()
//!        })
//!        .await?;
//!    println!("{resp:?}");
//! }
//! ```
//!
//! ### Listening to WS
//!
//! ```rust
//! async fn main() {
//!     let binance = Binance::with_key(&var("BINANCE_KEY")?);
//!     let listen_key = binance.request(StartUserDataStreamRequest {}).await?;
//!     let mut ws: BinanceWebsocket<UsdMWebsocketMessage> = BinanceWebsocket::new(&[
//!         listen_key.listen_key.as_str(),
//!         "ethusdt@aggTrade",
//!         "solusdt@bookTicker",
//!     ])
//!     .await?;
//!
//!     for _ in 0..10000 {
//!         let msg = ws.next().await.expect("ws exited")?;
//!         println!("{msg:?}");
//!     }
//! }
//! ```
//!
//! ## Module Structure
//!
//! Since this library heavily uses type, there are a plenty of structs defined in each module. The
//! organization of the structs follow the principle:
//! 1. REST related types are defined in the `rest` module (mainly request and responses).
//! 2. Websocket related types are defined in the `websocket` module (mainly websocket events).
//! 3. Common types like `OrderType` are defined in the `models` module.
//! 4. Binance distinguishes products like `Spot`, `USDM Futures`, so as our types. Types are further
//!    stored under the `usdm`, `coinm` and `spot` module under the `rest` and `websocket` module.

mod config;
mod error;
mod macros;
pub mod models;
mod parser;
pub mod rest;
pub mod websocket;

pub use config::Config;
pub use error::{BinanceError, BinanceResponseError};
#[cfg(feature = "zero-copy")]
pub use rest::C;
pub use rest::{Binance, RestResponse};
pub use websocket::BinanceWebsocket;
