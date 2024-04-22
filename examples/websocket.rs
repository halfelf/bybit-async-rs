use anyhow::Error;
use bybit_async::models::Product;
use bybit_async::{websocket::topics::WebsocketMessage, BybitWebsocket, Config};
use fehler::throws;
use futures::StreamExt;
use std::env;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    {
        let config = Config::new(Product::UsdMFutures);
        let mut ws: BybitWebsocket<WebsocketMessage> = BybitWebsocket::new(config).await?;
        println!("connected");
        ws.subscribe(["orderbook.1.BTCUSDT", "publicTrade.BTCUSDT"].to_vec())
            .await?;
        println!("subscribed");

        for _ in 0..500 {
            let msg = ws.next().await.expect("ws exited")?;
            println!("{msg:?}");
        }
    }

    {
        let api_key = env::var("API_KEY")?;
        let api_secret = env::var("API_SECRET")?;
        let config = Config::private_new(api_key, api_secret);
        let mut ws: BybitWebsocket<WebsocketMessage> = BybitWebsocket::new(config).await?;
        println!("connected");
        ws.subscribe(["order"].to_vec()).await?;
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");   // connected success message
        println!("subscribed");
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");  // subscribed success message

        // order updates
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");
        let msg = ws.next().await.expect("ws exited")?;
        println!("{msg:?}");
    }
}
