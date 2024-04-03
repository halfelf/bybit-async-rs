use anyhow::Error;
use bybit_async::{websocket::{topics::WebsocketMessage, BybitWebsocket}, Config};
use fehler::throws;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

#[throws(Error)]
#[tokio::test]
async fn ws_user_stream() {
    env_logger::init();

    let config = Config::default();
    let mut ws: BybitWebsocket<WebsocketMessage> = BybitWebsocket::new(
        config,
        &["position", "order", "execution"]
    ).await?;

    let fut = timeout(Duration::from_secs(5), ws.next());
    let msg = fut.await?.expect("ws exited")?;
    match msg {
        WebsocketMessage::UserOrderUpdate(orders) => println!("{orders:?}"),
        _ => unreachable!(),
    }
}
