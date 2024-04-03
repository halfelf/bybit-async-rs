use crate::{
    error::BybitError::{self, *},
    models::{OrderStatus, OrderType, Side, TimeInForce, OrderBookType},
    websocket::ParseMessage,
};
use fehler::{throw, throws};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum WebsocketMessage {
    Ping,

    // User Data Stream
    UserOrderUpdate(UserOrderUpdate),
    // UserPositionUpdate(UserPositionUpdate),
    // UserExecutionUpdate(UserExecutionUpdate),

    // Market Stream
    PublicTrade(PublicTrade),
    OrderBook(OrderBook),
}

impl ParseMessage for WebsocketMessage {
    #[throws(BybitError)]
    fn parse(stream: &str, data: &str) -> Self {
        if stream.starts_with("orderbook") {
            Self::OrderBook(from_str(data)?)
        } else if stream.starts_with("publicTrade") {
            Self::PublicTrade(from_str(data)?)
        } else if stream.starts_with("order") {
            Self::PublicTrade(from_str(data)?)
        } else {
            throw!(UnknownStream(stream.into()))
        }
    }

    fn ping() -> Self {
        Self::Ping
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserOrderUpdate {
    pub id: String,
    pub topic: String,
    pub creation_time: u64,
    pub data: Vec<OrderUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
// https://bybit-exchange.github.io/docs/zh-TW/v5/websocket/private/order
pub struct OrderUpdate {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    pub block_trade_id: u64,
    pub side: Side,
    pub position_idx: u8,
    pub order_status: OrderStatus,
    pub cancel_type: String,
    pub reject_reason: String,
    pub time_in_force: TimeInForce,
    pub is_leverage: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub avg_price: Decimal,
    pub leaves_qty: Decimal,
    pub leaves_value: Decimal,
    pub cum_exec_qty: Decimal,
    pub cum_exec_value: Decimal,
    pub cum_exec_fee: Decimal,
    pub order_type: OrderType,
    pub stop_order_type: String,
    pub order_iv: String,
    pub trigger_price: Decimal,
    pub take_profit: Decimal,
    pub stop_loss: Decimal,
    pub trigger_by: String,
    pub tp_trigger_by: String,
    pub sl_trigger_by: String,
    pub trigger_direction: u8,
    pub place_type: String,
    pub last_price_on_created: Decimal,
    pub close_on_trigger: bool,
    pub reduce_only: bool,
    pub smp_group: u32,
    pub smp_type: String,
    pub smp_order_id: String,
    pub sl_limit_price: Decimal,
    pub tp_limit_price: Decimal,
    pub market_unit: String,
    pub created_time: u64,
    pub updated_time: u64,
    pub fee_currency: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
// https://bybit-exchange.github.io/docs/zh-TW/v5/websocket/public/orderbook
pub struct OrderBook {
    pub topic: String,

    #[serde(rename = "type")]
    pub type_: OrderBookType,

    pub ts: u64,
    pub data: OrderBookUpdate,

    pub cts: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBookUpdate {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b")]
    pub bids: Vec<Vec<Decimal>>,

    #[serde(rename = "a")]
    pub asks: Vec<Vec<Decimal>>,

    #[serde(rename = "u")]
    pub update_id: u64,

    pub seq: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
// https://bybit-exchange.github.io/docs/zh-TW/v5/websocket/public/trade
pub struct PublicTrade {
    pub topic: String,

    #[serde(rename = "type")]
    pub type_: String,

    pub ts: u64,

    pub data: Vec<PublicTradeUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicTradeUpdate {
    #[serde(rename = "T")]
    pub exchange_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "v")]
    pub qty: Decimal,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "L")]
    pub liquidity: String,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    pub is_block_trade: bool,
}
