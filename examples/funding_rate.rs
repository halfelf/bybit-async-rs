use anyhow::Error;
use bybit_async::{
    rest::{coinm, usdm},
    Bybit,
};
use chrono::{DateTime, Utc};
use fehler::throws;

#[throws(Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let bybit = Bybit::new();

    let resp = bybit
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

    let resp = bybit
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
