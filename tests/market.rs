use anyhow::Error;
use binance_async::{rest::usdm, Binance};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn exchange_information() {
    env_logger::init();

    let binance = Binance::new();
    let ai = binance.request(usdm::ExchangeInformationRequest {}).await?;
    println!("{ai:?}");
}
