# bybit-async

Unofficial Rust Library for the [Bybit API](https://github.com/bybit-exchange/bybit-official-api-docs) with Async/Await and ergonomic design.

[![Crates.io](https://img.shields.io/crates/v/bybit-async.svg)](https://crates.io/crates/bybit-async)
[![Build Status](https://img.shields.io/github/actions/workflow/status/dovahcrow/bybit-async-rs/ci.yml?style=flat-square)](https://github.com/dovahcrow/bybit-async-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

[Documentation](https://docs.rs/crate/bybit-async)

This library borrows code from Flavio Oliveira (wisespace-io)'s [work](https://github.com/wisespace-io/bybit-rs). Thanks for his excellent library!

This repo is at its early stage, not all requests/websockets are implemented.
However, the related mechanism is already there: adding support for new requests/websocket events
should only require [several lines of code](#missing-endpoints-you-can-add-it-easily). PRs are very welcomed!

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

## MSRV

Rust 1.60

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
bybit-async = 0.3
```

Examples located in the examples folder.
* `examples/websocket.rs`: websocket subscribing market data and user data.
* `examples/new_order_and_cancel.rs`: create a new order than then cancel it.

## Ergonomic Design

The design of this library follows the `struct-based Request/Response` pattern.
This makes the API requests easy to use and understand. 

For example, to make a new order, you need to fill the `OrderRequest` struct, which
is defined as:
```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewOrderRequest {
    pub symbol: String,
    pub qty: Decimal,
    pub price: Decimal,
    pub stop_price: Option<Decimal>,
    pub order_side: Side,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}
```
You can just fill in the fields you want to fill, and leave the rest to `Default`. e.g.
```rust
let req = NewOrderRequest {
    symbol: "btcusd".into(),
    qty: 3.try_into().unwrap(),
    price: 20000.try_into().unwrap(),
    ..Default::default()
};

let client = Bybit::new();
client.request(req).await?;
```

This avoids the library to have a plethora of methods for different parameter 
combinations.

The magic behind the convenience is the `Request` trait. For example, `OrderRequest`
has the `Request` implemented as:
```rust
impl Request for NewOrderRequest {
    const API: APIUrl = APIUrl::Spot;
    const ENDPOINT: &'static str = "/api/v3/order";
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    type Response = OrderInfo;
}
```
This associates necessary information to each request struct.

## Missing Endpoints? You Can Add it Easily!

Due to the amount of APIs Bybit provides, it is hard to cover everything for this
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

Or, to make it simpler, use the macro ([see in action](https://github.com/dovahcrow/bybit-async-rs/blob/master/src/rest/usdm/account.rs)):
```rust
crate::define_request! {
    Name => GetCurrentPositionMode;
    Product => Product::UsdMFutures;
    Method => Method::GET;
    Endpoint => "/fapi/v1/positionSide/dual";
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}
```
