use crate::models::{AssetInformation, Filter, Product, RateLimit};
use reqwest::Method;
use serde::{Deserialize, Serialize};

crate::define_request! {
    Name => ExchangeInformation;
    Product => Product::UsdMFutures;
    Method => Method::GET;
    Endpoint => "/fapi/v1/exchangeInfo";
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
