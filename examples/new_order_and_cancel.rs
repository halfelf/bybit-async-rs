use anyhow::Error;
use bybit_async::{
    models::{OrderType, Side, TimeInForce},
    rest::usdm,
    Bybit,
};
use fehler::throws;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BYBIT_KEY")?, &var("BYBIT_SECRET")?);
    let resp = bybit
        .request(usdm::NewOrderRequest {
            symbol: "ethusdt".into(),
            r#type: OrderType::Limit,
            side: Side::Buy,

            price: Decimal::from_f64(1500.),
            quantity: Decimal::from_f64(0.004),
            time_in_force: Some(TimeInForce::GTC),
            ..Default::default()
        })
        .await?;
    println!("{resp:?}");

    let resp = bybit
        .request(usdm::CancelOrderRequest {
            symbol: "ethusdt".into(),
            order_id: Some(resp.order_id),
            ..Default::default()
        })
        .await?;
    println!("{resp:?}");
}
