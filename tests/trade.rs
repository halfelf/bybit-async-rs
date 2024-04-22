use anyhow::Error;
use bybit_async::{rest, Bybit};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn cancel_all_open_orders() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BYBIT_KEY")?, &var("BYBIT_SECRET")?);
    let resp = bybit
        .request(rest::trade::CancelAllOpenOrdersRequest {
            symbol: "BTCUSDT".into(),
        })
        .await?;
    println!("{resp:?}");
}

#[throws(Error)]
#[tokio::test]
async fn auto_cancel_all_open_orders() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BYBIT_KEY")?, &var("BYBIT_SECRET")?);
    let resp = bybit
        .request(rest::trade::AutoCancelAllOpenOrdersRequest {
            symbol: "BTCUSDT".into(),
            countdown_time: 1000000,
        })
        .await?;
    println!("{resp:?}");
}
