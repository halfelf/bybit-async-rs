use std::{env::var, time::Duration};

use anyhow::Error;
use binance_async::{
    rest::StartUserDataStreamUsdMRequest,
    websocket::{BinanceWebsocket, UsdMWebsocketMessage},
    Binance,
};
use fehler::throws;
use futures::StreamExt;
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_userstream() {
    env_logger::init();

    let binance = Binance::with_key(&var("BINANCE_KEY")?);
    let listen_key = binance.request(StartUserDataStreamUsdMRequest {}).await?;
    let mut ws: BinanceWebsocket<UsdMWebsocketMessage> = BinanceWebsocket::new(&[
        listen_key.listen_key.as_str(),
        // "ethbtc@aggTrade",
        // "btcusd@aggTrade",
    ])
    .await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        UsdMWebsocketMessage::AggregateTrade(agg) => println!("{agg:?}"),
    }
}
