mod market;
mod order;

pub use self::{market::*, order::*};
use crate::parser::string_or_decimal;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Spot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    Spot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub buyer_commission: f32,
    pub seller_commission: f32,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub require_self_trade_prevention: bool,
    pub update_time: u64,
    pub account_type: AccountType,
    pub permissions: Vec<Permission>,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    #[serde(with = "string_or_decimal")]
    pub free: Decimal,
    #[serde(with = "string_or_decimal")]
    pub locked: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub qty: Decimal,

    // Never serialized.
    #[serde(skip_serializing, rename = "ignore")]
    _ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub qty: Decimal,

    // Never serialized.
    #[serde(skip_serializing, rename = "ignore")]
    _ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Ticker>),
}

#[derive(Debug, Clone)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub bid_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub bid_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub ask_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub ask_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub symbol: String,
    pub id: u64,
    pub order_id: u64,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub commission: Decimal,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub price_change: Decimal,
    #[serde(with = "string_or_decimal")]
    pub price_change_percent: Decimal,
    #[serde(with = "string_or_decimal")]
    pub weighted_avg_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub prev_close_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub last_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub bid_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub ask_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub open_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub high_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub low_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64, // For dummy symbol "123456", it is -1
    pub last_id: i64,  // Same as above
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct KlineSummary {
    pub open_time: i64,

    pub open: Decimal,

    pub high: Decimal,

    pub low: Decimal,

    pub close: Decimal,

    pub volume: Decimal,

    pub close_time: i64,

    pub quote_asset_volume: Decimal,

    pub number_of_trades: i64,

    pub taker_buy_base_asset_volume: Decimal,

    pub taker_buy_quote_asset_volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "V")]
    pub active_buy_volume: String,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,
    #[serde(skip_serializing, rename = "B")]
    pub ignore_me: String,
}

// {
//     "symbol": "ETHBTC",
//     "status": "TRADING",
//     "baseAsset": "ETH",
//     "baseAssetPrecision": 8,
//     "quoteAsset": "BTC",
//     "quotePrecision": 8,
//     "orderTypes": ["LIMIT", "MARKET"],
//     "icebergAllowed": false,
//     "filters": [{
//       "filterType": "PRICE_FILTER",
//       "minPrice": "0.00000100",
//       "maxPrice": "100000.00000000",
//       "tickSize": "0.00000100"
//     }, {
//       "filterType": "LOT_SIZE",
//       "minQty": "0.00100000",
//       "maxQty": "100000.00000000",
//       "stepSize": "0.00100000"
//     }, {
//       "filterType": "MIN_NOTIONAL",
//       "minNotional": "0.00100000"
//     }]
//   }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeFilter {
    ExchangeMaxNumOrders { limit: u64 },
    ExchangeMaxAlgoOrders { limit: u64 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NewOrderResponseType {
    Ack,
    Result,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderExecType {
    New,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRejectReason {
    None,
}
