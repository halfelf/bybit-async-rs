use anyhow::Error;
use binance_async::{
    rest::usdm::StartUserDataStreamRequest, websocket::UsdMWebsocketMessage, Binance,
    BinanceWebsocket,
};
use fehler::throws;
use futures::StreamExt;
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let binance = Binance::with_key(&var("BINANCE_KEY")?);
    let listen_key = binance.request(StartUserDataStreamRequest {}).await?;
    let mut ws: BinanceWebsocket<UsdMWebsocketMessage> = BinanceWebsocket::new(&[
        listen_key.listen_key.as_str(),
        "ethusdt@aggTrade",
        "solusdt@bookTicker",
    ])
    .await?;

    for _ in 0..10000 {
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");
    }
}
