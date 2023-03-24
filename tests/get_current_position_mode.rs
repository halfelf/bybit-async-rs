use anyhow::Error;
use binance_async::{rest::GetCurrentPositionModeRequest, Binance};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_current_position_mode() {
    env_logger::init();

    let binance = Binance::with_credential(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let ai = binance.request(GetCurrentPositionModeRequest {}).await?;
    println!("{ai:?}");
}
