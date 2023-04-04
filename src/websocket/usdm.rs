use super::AggregateTrade;
use crate::{
    error::BinanceError::{self, *},
    models::{ExecutionType, OrderStatus, OrderType, Product, Side, TimeInForce},
    parser::{string_or_decimal, string_or_decimal_opt},
    websocket::ParseMessage,
};
use fehler::{throw, throws};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum WebsocketMessage {
    // User Data Stream
    UserOrderUpdate(OrderUpdate),
    UserAccountUpdate(AccountUpdate),
    UserDataStreamExpired,
    // Market Stream
    AggregateTrade(AggregateTrade),
    BookTicker(BookTicker),
    // Trade(TradeMessage),
    // Candlestick(CandelStickMessage),
    // MiniTicker(MiniTicker),
    // MiniTickerAll(Vec<MiniTicker>),
    // Ticker(Ticker),
    // TickerAll(Vec<Ticker>),
    // OrderBook(OrderBook),
    // Depth(Depth),
}

impl ParseMessage for WebsocketMessage {
    const PRODUCT: Product = Product::UsdMFutures;

    #[throws(BinanceError)]
    fn parse(stream: &str, data: &str) -> Self {
        if stream.ends_with("@aggTrade") {
            Self::AggregateTrade(from_str(data)?)
        } else if stream.contains("@markPrice") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.starts_with("!markPrice@arr") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.contains("@kline_") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.contains("@continuousKline_") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@miniTicker") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!miniTicker@arr" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@ticker") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!ticker@arr" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@bookTicker") || stream == "!bookTicker" {
            Self::BookTicker(from_str(data)?)
        } else if stream.ends_with("@forceOrder") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!forceOrder@arr" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@depth") || stream.contains("@depth@") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.contains("@depth") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.ends_with("@compositeIndex") {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream == "!contractInfo" {
            throw!(StreamNotImplemented(stream.into()))
        } else if stream.len() == 64 {
            // is a user stream
            let value: UserDataStreamEvent = from_str(data)?;
            match value.event_type.as_ref() {
                "ACCOUNT_UPDATE" => Self::UserAccountUpdate(
                    value.account.ok_or(EmptyUserDataStream(value.event_type))?,
                ),
                "ORDER_TRADE_UPDATE" => {
                    Self::UserOrderUpdate(value.order.ok_or(EmptyUserDataStream(value.event_type))?)
                }
                "listenKeyExpired" => Self::UserDataStreamExpired,
                _ => throw!(UserDataStreamEventNotImplemented(value.event_type)),
            }
        } else {
            throw!(UnknownStream(stream.into()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: Option<u64>,

    #[serde(rename = "o")]
    pub order: Option<OrderUpdate>,

    #[serde(rename = "a")]
    pub account: Option<AccountUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub new_client_order_id: String,

    #[serde(rename = "S")]
    pub side: Side,

    #[serde(rename = "o")]
    pub order_type: OrderType,

    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,

    #[serde(rename = "q", with = "string_or_decimal")]
    pub qty: Decimal,

    #[serde(rename = "p", with = "string_or_decimal")]
    pub price: Decimal,

    #[serde(rename = "ap", with = "string_or_decimal")]
    pub average_price: Decimal,

    #[serde(rename = "sp", with = "string_or_decimal")]
    pub stop_price: Decimal,

    #[serde(rename = "x")]
    pub execution_type: ExecutionType,

    #[serde(rename = "X")]
    pub order_status: OrderStatus,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l", with = "string_or_decimal")]
    pub qty_last_filled_trade: Decimal,

    #[serde(rename = "z", with = "string_or_decimal")]
    pub accumulated_qty_filled_trades: Decimal,

    #[serde(rename = "L", with = "string_or_decimal")]
    pub price_last_filled_trade: Decimal,

    #[serde(skip, rename = "N")]
    pub asset_commisioned: Option<String>,

    #[serde(rename = "n", with = "string_or_decimal_opt")]
    pub commission: Option<Decimal>,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "t")]
    pub trade_id: i64,

    #[serde(rename = "b", with = "string_or_decimal")]
    pub bids_notional: Decimal,

    #[serde(rename = "a", with = "string_or_decimal")]
    pub ask_notional: Decimal,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(rename = "R")]
    pub is_reduce_only: bool,

    #[serde(rename = "wt")]
    pub stop_price_working_type: String,

    #[serde(rename = "ot")]
    pub original_order_type: String,

    #[serde(rename = "ps")]
    pub position_side: String,

    #[serde(rename = "cp")]
    pub close_all: Option<bool>,

    #[serde(rename = "AP")]
    pub activation_price: Option<String>,

    #[serde(rename = "cr")]
    pub callback_rate: Option<String>,

    #[serde(rename = "pP")]
    pub pp_ignore: bool,

    #[serde(rename = "si")]
    pub si_ignore: i32,

    #[serde(rename = "ss")]
    pub ss_ignore: i32,

    #[serde(rename = "rp")]
    pub realized_profit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(rename = "m")]
    pub reason: String,

    #[serde(rename = "B")]
    pub balances: Vec<AccountUpdateBalance>,

    #[serde(rename = "P")]
    pub positions: Vec<AccountUpdatePosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", with = "string_or_decimal")]
    pub wallet_balance: Decimal,
    #[serde(rename = "cw", with = "string_or_decimal")]
    pub cross_wallet_balance: Decimal,
    #[serde(rename = "bc", with = "string_or_decimal")]
    pub balance_change: Decimal, // Balance Change except PnL and Commission
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdatePosition {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", with = "string_or_decimal")]
    pub position_amount: Decimal,
    #[serde(rename = "ep", with = "string_or_decimal")]
    pub entry_price: Decimal,
    #[serde(rename = "cr", with = "string_or_decimal")]
    pub accumulated_realized: Decimal, // (Pre-fee) Accumulated Realized
    #[serde(rename = "up", with = "string_or_decimal")]
    pub unrealized_pnl: Decimal,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_wallet: String,
    #[serde(rename = "ps")]
    pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    #[serde(rename = "u")]
    pub update_id: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b", with = "string_or_decimal")]
    pub best_bid: Decimal,

    #[serde(rename = "B", with = "string_or_decimal")]
    pub best_bid_qty: Decimal,

    #[serde(rename = "a", with = "string_or_decimal")]
    pub best_ask: Decimal,

    #[serde(rename = "A", with = "string_or_decimal")]
    pub best_ask_qty: Decimal,
}
