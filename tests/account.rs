use anyhow::Error;
use bybit_async::{
    rest::{spot, usdm},
    Bybit,
};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_account_spot() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BYBIT_KEY")?, &var("BYBIT_SECRET")?);
    let resp = bybit.request(spot::GetAccountRequest {}).await?;
    println!("{resp:?}");
}

#[throws(Error)]
#[tokio::test]
async fn get_account_usdm() {
    env_logger::init();

    let bybit = Bybit::with_key_and_secret(&var("BYBIT_KEY")?, &var("BYBIT_SECRET")?);
    let resp = bybit.request(usdm::AccountInformationV2Request {}).await?;
    println!("{resp:?}");
}
