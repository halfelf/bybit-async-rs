use anyhow::Error;
use bybit_async::{rest::spot, Bybit};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn ping() {
    env_logger::init();

    let bybit = Bybit::new();
    let resp = bybit.request(spot::PingRequest {}).await?;
    println!("{resp:?}");
}
