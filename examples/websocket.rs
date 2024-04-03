use anyhow::Error;
use bybit_async::{
    rest::usdm::StartUserDataStreamRequest, websocket::usdm::WebsocketMessage, Bybit,
    BybitWebsocket,
};
use fehler::throws;
use futures::StreamExt;
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let bybit = Bybit::with_key(&var("BYBIT_KEY")?);
    let listen_key = bybit.request(StartUserDataStreamRequest {}).await?;
    let mut ws: BybitWebsocket<WebsocketMessage> = BybitWebsocket::new(&[
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
