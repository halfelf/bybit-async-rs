use anyhow::Error;
use binance_async::{
    rest::{spot, usdm},
    Binance,
};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_account_spot() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = binance.request(spot::GetAccountRequest {}).await?;
    let resp = &*resp;
    println!("{resp:?}");
}

#[throws(Error)]
#[tokio::test]
async fn get_account_usdm() {
    env_logger::init();

    let binance = Binance::with_key_and_secret(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let resp = binance
        .request(usdm::AccountInformationV2Request {})
        .await?;
    let resp = &*resp;
    println!("{resp:?}");
}
