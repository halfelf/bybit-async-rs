use anyhow::Error;
use bybit_async::models::Product;
use bybit_async::websocket::{topics::WebsocketMessage, BybitWebsocket};
use bybit_async::Config;
use fehler::throws;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_public() {
    env_logger::init();
    let config = Config::new(Product::UsdMFutures);
    let mut ws: BybitWebsocket<WebsocketMessage> = BybitWebsocket::new(config).await?;
    println!("connected");
    ws.subscribe(["orderbook.1.BTCUSDT", "publicTrade.BTCUSDT"].to_vec()).await?;
    println!("subscribed");

    for _ in 0..100 {
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");
    }
}
