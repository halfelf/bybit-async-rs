use anyhow::Error;
use bybit_async::{rest, Bybit};
use fehler::throws;

#[throws(Error)]
#[tokio::test]
async fn ping() {
    env_logger::init();

    // let bybit = Bybit::new();
    // let resp = bybit.request(rest::PingRequest {}).await?;
    // println!("{resp:?}");
}
