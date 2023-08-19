use crate::models::{AssetInformation, Filter, Product, RateLimit};
use crate::parser::string_or_decimal;
use chrono::serde::{ts_milliseconds, ts_milliseconds_option};
use chrono::{DateTime, Utc};
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

crate::define_request! {
    Name => ExchangeInformation;
    Product => Product::CoinMFutures;
    Method => Method::GET;
    Endpoint => "/dapi/v1/exchangeInfo";
    Signed => false;
    Request => {};
    Response => {
        pub timezone: String,
        pub futures_type: String,
        pub rate_limits: Vec<RateLimit>,
        pub server_time: u64,
        pub assets: Vec<AssetInformation>,
        pub symbols: Vec<Symbol>
    };
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub status: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub filters: Vec<Filter>,
}

crate::define_request! {
    Name => FundingRate;
    Product => Product::CoinMFutures;
    Method => Method::GET;
    Endpoint => "/dapi/v1/fundingRate";
    Signed => false;
    Request => {
        pub symbol: String,
        #[serde(with = "ts_milliseconds_option")]
        pub start_time: Option<DateTime<Utc>>,
        #[serde(with = "ts_milliseconds_option")]
        pub end_time: Option<DateTime<Utc>>,
        pub limit: Option<u64>,
    };
    Response => Vec<FundingRate>;
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub funding_rate: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub funding_time: DateTime<Utc>,
}
