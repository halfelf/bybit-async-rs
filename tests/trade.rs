use anyhow::Error;
use bybit_async::{rest::usdm, Bybit};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn cancel_all_open_orders() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = bybit
        .request(usdm::CancelAllOpenOrdersRequest {
            symbol: "BTCUSDT".into(),
        })
        .await?;
    println!("{resp:?}");
}

#[throws(Error)]
#[tokio::test]
async fn auto_cancel_all_open_orders() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = bybit
        .request(usdm::AutoCancelAllOpenOrdersRequest {
            symbol: "BTCUSDT".into(),
            countdown_time: 1000000,
        })
        .await?;
    println!("{resp:?}");
}
