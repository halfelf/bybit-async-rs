use anyhow::Error;
use bybit_async::models::Product;
use bybit_async::{websocket::topics::WebsocketMessage, BybitWebsocket, Config};
use fehler::throws;
use futures::StreamExt;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::new(Product::UsdMFutures);
    let mut ws: BybitWebsocket<WebsocketMessage> =
        BybitWebsocket::new(config, &["orderbook.1.BTCUSDT", "publicTrade.BTCUSDT"]).await?;

    for _ in 0..10000 {
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");
    }
}
