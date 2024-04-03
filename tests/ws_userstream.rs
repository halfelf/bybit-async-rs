use anyhow::Error;
use bybit_async::{
    rest::usdm::StartUserDataStreamRequest,
    websocket::{topics::WebsocketMessage, BybitWebsocket},
    Bybit,
};
use fehler::throws;
use futures::StreamExt;
use std::{env::var, time::Duration};
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_user_stream() {
    env_logger::init();

    let bybit = Bybit::with_key(&var("BYBIT_KEY")?);
    let mut ws: BybitWebsocket<WebsocketMessage> =
        BybitWebsocket::new(&[]).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::UserOrderUpdate(orders) => println!("{orders:?}"),
        _ => unreachable!(),
    }
}
