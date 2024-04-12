use crate::parser::string_or_decimal;
use crate::parser::string_or_decimal_opt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod account;
pub mod market;
pub mod order;

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
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
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
