use anyhow::Error;
use bybit_async::{rest::usdm, Bybit};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn exchange_information() {
    env_logger::init();

    let bybit = Bybit::new();
    let resp = bybit.request(usdm::ExchangeInformationRequest {}).await?;
    println!("{resp:?}");
}
