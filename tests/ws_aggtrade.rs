use std::time::Duration;

use anyhow::Error;
use binance_async::{
    websocket::{BinanceWebsocket, SpotWebsocketMessage},
    Binance,
};
use fehler::throws;
use futures::StreamExt;
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_aggtrade() {
    env_logger::init();

    let binance = Binance::new();
    let mut ws: BinanceWebsocket<SpotWebsocketMessage> = binance
        .subscribe(&["ethbtc@aggTrade", "btcusd@aggTrade"])
        .await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        SpotWebsocketMessage::AggregateTrade(agg) => println!("{agg:?}"),
    }
}
