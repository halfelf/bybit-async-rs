use anyhow::Error;
use binance_async::{
    rest::{coinm, usdm},
    Binance,
};
use chrono::{DateTime, Utc};
use fehler::throws;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let binance = Binance::new();

    let resp = binance
        .request(usdm::FundingRateRequest {
            symbol: Some("SOLUSDT".into()),
            start_time: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            limit: Some(1000),
            ..Default::default()
        })
        .await?;
    println!("{resp:?}");

    let resp = binance
        .request(coinm::FundingRateRequest {
            symbol: "SOLUSD_PERP".into(),
            start_time: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            limit: Some(1000),
            ..Default::default()
        })
        .await?;
    println!("{resp:?}");
}
