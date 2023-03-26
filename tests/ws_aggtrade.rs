use anyhow::Error;
use binance_async::websocket::{spot::WebsocketMessage, BinanceWebsocket};
use fehler::throws;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_aggtrade() {
    env_logger::init();

    let mut ws: BinanceWebsocket<WebsocketMessage> =
        BinanceWebsocket::new(&["ethbtc@aggTrade", "btcusd@aggTrade"]).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::AggregateTrade(agg) => println!("{agg:?}"),
        _ => unreachable!(),
    }
}
