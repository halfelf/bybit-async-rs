use anyhow::Error;
use binance_async::{rest::spot, Binance};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_account() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let ai = binance.request(spot::GetAccountRequest {}).await?;
    println!("{ai:?}");
}
