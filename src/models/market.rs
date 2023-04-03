use crate::parser::{string_or_decimal, string_or_decimal_opt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    interval: Interval,
    interval_num: usize,
    limit: usize,
    rate_limit_type: RateLimitType,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetInformation {
    asset: String,
    margin_available: bool,
    auto_asset_exchange: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Interval {
    Minute,
}

impl Default for Interval {
    fn default() -> Self {
        Interval::Minute
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
}

impl Default for RateLimitType {
    fn default() -> Self {
        RateLimitType::RequestWeight
    }
}

// {
//       "filterType": "LOT_SIZE",
//       "minQty": "0.00100000",
//       "maxQty": "100000.00000000",
//       "stepSize": "0.00100000"
//     }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde(with = "string_or_decimal")]
        min_price: Decimal,
        #[serde(with = "string_or_decimal")]
        max_price: Decimal,
        #[serde(with = "string_or_decimal")]
        tick_size: Decimal,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(with = "string_or_decimal")]
        multiplier_up: Decimal,
        #[serde(with = "string_or_decimal")]
        multiplier_down: Decimal,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    #[serde(rename_all = "camelCase")]
    PercentPriceBySide {
        #[serde(with = "string_or_decimal")]
        bid_multiplier_up: Decimal,
        #[serde(with = "string_or_decimal")]
        bid_multiplier_down: Decimal,
        #[serde(with = "string_or_decimal")]
        ask_multiplier_up: Decimal,
        #[serde(with = "string_or_decimal")]
        ask_multiplier_down: Decimal,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde(with = "string_or_decimal")]
        min_qty: Decimal,
        #[serde(with = "string_or_decimal")]
        max_qty: Decimal,
        #[serde(with = "string_or_decimal")]
        step_size: Decimal,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(default, with = "string_or_decimal_opt")]
        notional: Option<Decimal>,
        #[serde(default, with = "string_or_decimal_opt")]
        min_notional: Option<Decimal>,
        apply_to_market: Option<bool>,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    Notional {
        #[serde(default, with = "string_or_decimal_opt")]
        notional: Option<Decimal>,
        #[serde(default, with = "string_or_decimal_opt")]
        min_notional: Option<Decimal>,
        apply_to_market: Option<bool>,
        avg_price_mins: Option<f64>,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: Option<u16> },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: Option<u16> },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u16 },
    #[serde(rename = "MAX_POSITION")]
    #[serde(rename_all = "camelCase")]
    MaxPosition { max_position: String },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        #[serde(with = "string_or_decimal")]
        min_qty: Decimal,
        #[serde(with = "string_or_decimal")]
        max_qty: Decimal,
        #[serde(with = "string_or_decimal")]
        step_size: Decimal,
    },
    #[serde(rename = "TRAILING_DELTA")]
    #[serde(rename_all = "camelCase")]
    TrailingData {
        min_trailing_above_delta: Option<u16>,
        max_trailing_above_delta: Option<u16>,
        min_trailing_below_delta: Option<u16>,
        max_trailing_below_delta: Option<u16>,
    },
}
