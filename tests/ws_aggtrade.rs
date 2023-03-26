use std::time::Duration;

use anyhow::Error;
use binance_async::websocket::{BinanceWebsocket, SpotWebsocketMessage};
use fehler::throws;
use futures::StreamExt;
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_aggtrade() {
    env_logger::init();

    let mut ws: BinanceWebsocket<SpotWebsocketMessage> =
        BinanceWebsocket::new(&["ethbtc@aggTrade", "btcusd@aggTrade"]).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        SpotWebsocketMessage::AggregateTrade(agg) => println!("{agg:?}"),
    }
}
