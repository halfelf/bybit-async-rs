use anyhow::Error;
use bybit_async::{rest, Bybit};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn exchange_information() {
    env_logger::init();

    // let bybit = Bybit::new();
    // let resp = bybit.request(rest::trade::ExchangeInformationRequest {}).await?;
    // println!("{resp:?}");
}
