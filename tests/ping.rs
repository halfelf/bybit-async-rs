use anyhow::Error;
use binance_async::{rest::PingSpotRequest, Binance};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn ping() {
    env_logger::init();

    let binance = Binance::new();
    let ai = binance.request(PingSpotRequest {}).await?;
    println!("{ai:?}");
}
