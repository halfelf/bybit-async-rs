use anyhow::Error;
use bybit_async::{rest::usdm, Bybit};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_current_position_mode() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = bybit
        .request(usdm::GetCurrentPositionModeRequest {})
        .await?;
    println!("{resp:?}");
}
