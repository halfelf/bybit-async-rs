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
    ws.subscribe(["orderbook.1.BTCUSDT", "publicTrade.BTCUSDT"].to_vec()).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::PublicTrade(trades) => println!("{trades:?}"),
        WebsocketMessage::OrderBook(orderbook) => println!("{orderbook:?}"),
        _ => unreachable!(),
    }
}
