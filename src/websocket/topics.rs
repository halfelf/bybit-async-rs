use crate::{
    error::BybitError::{self, *},
    models::{OrderStatus, OrderType, Side, TimeInForce},
    websocket::ParseMessage,
};
use fehler::{throw, throws};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString};

#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum WebsocketMessage {
    Ping,
    SubscribeSuccess(SubscribeSuccess),

    // User Data Stream
    UserOrderUpdate(Vec<UserOrderUpdate>),
    // todo!(UserPositionUpdate(UserPositionUpdate),)
    // todo!(UserExecutionUpdate(UserExecutionUpdate),)

    // Market Stream
    PublicTrade(Vec<PublicTradeUpdate>),
    OrderBook(OrderBook),
    // todo!(tickers),
    // todo!(kline),
    // todo!(liquidation),
}

impl ParseMessage for WebsocketMessage {
    #[throws(BybitError)]
    fn parse(topic: &str, data: &str) -> Self {
        if topic.starts_with("orderbook") {
            Self::OrderBook(from_str(data)?)
        } else if topic.starts_with("publicTrade") {
            Self::PublicTrade(from_str(data)?)
        } else if topic.starts_with("order") {
            Self::UserOrderUpdate(from_str(data)?)
        } else {
            throw!(UnknownStream(topic.into()))
        }
    }

    fn parse_succ(succ: &str) -> Result<Self, BybitError> {
        Ok(Self::SubscribeSuccess(from_str(succ)?))
    }

    fn ping() -> Self {
        Self::Ping
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
// https://bybit-exchange.github.io/docs/zh-TW/v5/websocket/private/order
pub struct UserOrderUpdate {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub block_trade_id: Option<u64>,
    pub side: Side,
    pub position_idx: u8,
    pub order_status: OrderStatus,
    pub cancel_type: String,
    pub reject_reason: String,
    pub time_in_force: TimeInForce,
    pub is_leverage: String,
    pub price: Decimal,
    pub qty: Decimal,
    #[serde_as(as = "NoneAsEmptyString")]
    pub avg_price: Option<Decimal>,
    pub leaves_qty: Decimal,
    pub leaves_value: Decimal,
    pub cum_exec_qty: Decimal,
    pub cum_exec_value: Decimal,
    pub cum_exec_fee: Decimal,
    pub order_type: OrderType,
    pub stop_order_type: String,
    pub order_iv: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub trigger_price: Option<Decimal>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub take_profit: Option<Decimal>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub stop_loss: Option<Decimal>,
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
    #[serde_as(as = "DisplayFromStr")]
    pub created_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub updated_time: u64,
    pub fee_currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// https://bybit-exchange.github.io/docs/zh-TW/v5/websocket/public/orderbook
pub struct OrderBook {
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
    pub tick_direction: String,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    pub is_block_trade: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// https://bybit-exchange.github.io/docs/zh-TW/v5/ws/connect
pub struct SubscribeSuccess {
    pub success: bool,
    pub ret_msg: String,
    pub op: String,
    pub conn_id: String,
    pub req_id: Option<String>,
}
