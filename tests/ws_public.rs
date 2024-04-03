use anyhow::Error;
use bybit_async::websocket::{topics::WebsocketMessage, BybitWebsocket};
use fehler::throws;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;
use bybit_async::Config;
use bybit_async::models::Product;

#[throws(Error)]
#[tokio::test]
async fn ws_public() {
    env_logger::init();

    let config = Config::new(Product::UsdMFutures);

    let mut ws: BybitWebsocket<WebsocketMessage> =
        BybitWebsocket::new(
            config,
            &["orderbook.1.BTCUSDT", "publicTrade.BTCUSDT"]
        ).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::PublicTrade(trades) => println!("{trades:?}"),
        _ => unreachable!(),
    }
}
