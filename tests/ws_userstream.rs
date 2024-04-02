use anyhow::Error;
use bybit_async::{
    rest::usdm::StartUserDataStreamRequest,
    websocket::{usdm::WebsocketMessage, BybitWebsocket},
    Bybit,
};
use fehler::throws;
use futures::StreamExt;
use std::{env::var, time::Duration};
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_userstream() {
    env_logger::init();

    let bybit = Bybit::with_key(&var("BINANCE_KEY")?);
    let listen_key = bybit.request(StartUserDataStreamRequest {}).await?;
    let mut ws: BybitWebsocket<WebsocketMessage> =
        BybitWebsocket::new(&[listen_key.listen_key.as_str()]).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::AggregateTrade(agg) => println!("{agg:?}"),
        _ => unreachable!(),
    }
}
