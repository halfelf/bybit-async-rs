use anyhow::Error;
use binance_async::{
    model::{OrderSide, OrderType, TimeInForce},
    rest::usdm,
    Binance,
};
use fehler::throws;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = binance
        .request(usdm::NewOrderRequest {
            symbol: "ethusdt".into(),
            r#type: OrderType::Limit,
            side: OrderSide::Buy,

            price: Decimal::from_f64(1500.),
            quantity: Decimal::from_f64(0.004),
            time_in_force: Some(TimeInForce::GTC),
            ..Default::default()
        })
        .await?;
    println!("{resp:?}");

    let resp = binance
        .request(usdm::CancelOrderRequest {
            symbol: "ethusdt".into(),
            order_id: Some(resp.order_id),
            ..Default::default()
        })
        .await?;
    println!("{resp:?}");
}
