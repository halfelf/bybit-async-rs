use crate::parser::{string_or, string_or_decimal};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: String,
    #[serde(with = "string_or_decimal")]
    pub wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub unrealized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub margin_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub maint_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub position_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub open_order_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_withdraw_amount: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(with = "string_or_decimal")]
    pub cross_un_pnl: Decimal,
    #[serde(with = "string_or_decimal")]
    pub available_balance: Decimal,
    #[serde(with = "string_or")]
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub symbol: String,
    #[serde(with = "string_or_decimal")]
    pub initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub maint_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub unrealized_profit: Decimal,
    #[serde(with = "string_or_decimal")]
    pub position_initial_margin: Decimal,
    #[serde(with = "string_or_decimal")]
    pub open_order_initial_margin: Decimal,
    pub leverage: String,
    #[serde(with = "string_or")]
    pub isolated: bool,
    #[serde(with = "string_or_decimal")]
    pub entry_price: Decimal,
    #[serde(with = "string_or_decimal")]
    pub max_notional: Decimal,
    pub position_side: String,
    #[serde(with = "string_or_decimal", rename = "positionAmt")]
    pub position_amount: Decimal,
    #[serde(with = "string_or_decimal")]
    pub notional: Decimal,
    #[serde(with = "string_or_decimal")]
    pub isolated_wallet: Decimal,
    pub update_time: u64,
    #[serde(with = "string_or_decimal")]
    pub bid_notional: Decimal,
    #[serde(with = "string_or_decimal")]
    pub ask_notional: Decimal,
}
