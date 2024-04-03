use crate::parser::string_or_decimal;
use crate::parser::string_or_decimal_opt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod spot;
pub mod usdm;

#[derive(Copy, Clone, Debug)]
pub enum Product {
    Spot,
    UsdMFutures,
    CoinMFutures,
    EuropeanOptions,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Self {
        Self::Buy
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    PostOnly,
}

impl Default for TimeInForce {
    fn default() -> Self {
        Self::GTC
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum OrderType {
    Market,
    Limit,
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
    PartialStopLoss,
    tpslOrder,
    OcoOrder,
    MmRateClose,
    BidirectionalTpslOrder,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    /// The order has been accepted into the engine.
    New,
    /// The order has been canceled by the user.
    Canceled,
    /// Liquidation Execution (Futures only)
    Calculated,
    /// (currently unused)
    Replaced,
    /// The order has been rejected and was not processed (This message appears only with Cancel Replace Orders wherein the new order placement is rejected but the request to cancel request succeeds.)
    Rejected,
    /// Part of the order or all of the order's quantity has filled.
    Trade,
    /// The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill) or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance).
    Expired,
    /// The order has expired due to STP trigger.
    TradePrevention,
}

// {
//       "rateLimitType": "ORDERS",
//       "interval": "DAY",
//       "limit": 100000
//     }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: Interval,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    Orders,
    RequestWeight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Interval {
    Second,
    Minute,
    Day,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetInformation {
    asset: String,
    margin_available: bool,
    auto_asset_exchange: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderBookType {
    Snapshot,
    Delta,
}
