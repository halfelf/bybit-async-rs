use crate::parser::{string_or_decimal, string_or_decimal_opt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    pub orig_qty: String,
    pub executed_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub side: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub stop_price: Decimal,
    pub iceberg_qty: String,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u64,
    pub client_order_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: Option<i64>,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub orig_qty: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub executed_qty: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub cummulative_quote_qty: Decimal,
    #[serde(with = "rust_decimal::serde::str", default = "default_stop_price")]
    pub stop_price: Decimal,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    pub fills: Option<Vec<FillInfo>>,
}

fn default_stop_price() -> Decimal {
    Decimal::ZERO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillInfo {
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub qty: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: Decimal,
    pub commission_asset: String,
    pub trade_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Buy
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub client_order_id: String,
    #[serde(with = "string_or_decimal")]
    pub cum_qty: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cum_quote: Decimal,
    #[serde(with = "string_or_decimal")]
    pub executed_qty: Decimal,
    pub order_id: u64,
    #[serde(with = "string_or_decimal")]
    pub orig_qty: Decimal,
    pub orig_type: String,
    #[serde(with = "string_or_decimal")]
    pub price: Decimal,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_decimal")]
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    #[serde(with = "string_or_decimal_opt")]
    pub activate_price: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_or_decimal_opt")]
    pub price_rate: Option<Decimal>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}
