# binance-async

Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs) with Async/Await and ergonomic design.

[![Crates.io](https://img.shields.io/crates/v/binance-async.svg)](https://crates.io/crates/binance-async)
[![Build Status](https://img.shields.io/github/actions/workflow/status/dovahcrow/binance-async-rs/ci.yml?style=flat-square)](https://github.com/dovahcrow/binance-async-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

[Documentation](https://docs.rs/crate/binance-async)

This library borrows code from Flavio Oliveira (wisespace-io)'s [work](https://github.com/wisespace-io/binance-rs). Thanks for his excellent library!

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

## MSRV

Rust 1.60

## Usage

This library is just revived. Please use the git version for now.

~~Add this to your Cargo.toml~~

```toml
[dependencies]
binance-async = 0.2
```

Examples located in the examples folder.

## Ergonomic Design

The design of this library follows the `struct-based Request/Response` pattern.
This makes the API requests easy to use and understand. 

For example, to make a new order, you need to fill the `OrderRequest` struct, which
is defined as:
```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}
```
You can just fill in the fields you want to fill, and leave the rest to `Default`. e.g.
```rust
let req = OrderRequest {
    symbol: "btcusd".into(),
    qty: 3.,
    price: 20000.,
    ..Default::default()
};

let client = Binance::new();
client.request(req).await?;
```

This avoids the library to have a plethora of methods for different parameter 
combinations.

The magic behind the convenience is the `Request` trait. For example, `OrderRequest`
has the `Request` implemented as:
```rust
impl Request for OrderRequest {
    const API: APIUrl = APIUrl::Spot;
    const ENDPOINT: &'static str = "/api/v3/order";
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    type Response = OrderInfo;
}
```
This associates necessary information to each request struct.

## Missing Endpoints? You Can Add it Easily!

Due to the amount of APIs Binance provides, it is hard to cover everything for this
library. 

However, since this library uses the `struct-based Request/Response` pattern, adding a new
request is easy. You only need to add a new `Request` struct and a new `Response` struct
into the source code and implement the `Request` trait to the newly added `Request`
struct.

For example, adding `GET /fapi/v1/positionSide/dual` is just

```rust
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GetCurrentPositionModeRequest {}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentPositionModeResponse {
    dual_side_position: bool,
}

impl Request for GetCurrentPositionModeRequest {
    const API: APIUrl = APIUrl::UsdMFutures;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    type Response = GetCurrentPositionModeResponse;
}
```

Or, to make it simpler, use the macro ([see in action](https://github.com/dovahcrow/binance-async-rs/blob/master/src/rest/usdm/account.rs)):
```rust
crate::define_request! {
    Name => GetCurrentPositionMode;
    API => APIUrl::UsdMFutures;
    Endpoint => "/fapi/v1/positionSide/dual";
    Method => Method::GET;
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}
```
