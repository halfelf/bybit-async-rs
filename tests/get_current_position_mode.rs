use anyhow::Error;
use binance_async::{rest::usdm, Binance};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_current_position_mode() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = binance
        .request(usdm::GetCurrentPositionModeRequest {})
        .await?;
    let resp = &*resp;
    println!("{resp:?}");
}
