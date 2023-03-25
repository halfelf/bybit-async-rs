use super::{APIUrl, Request};
use crate::model::{AccountInformation, OrderInfo, OrderSide, OrderType, TimeInForce};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct GetAccountRequest {}

impl Request for GetAccountRequest {
    const API: APIUrl = APIUrl::Spot;
    const ENDPOINT: &'static str = "/api/v3/account";
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    type Response = AccountInformation;
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

impl Request for OrderRequest {
    const API: APIUrl = APIUrl::Spot;
    const ENDPOINT: &'static str = "/api/v3/order";
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    type Response = OrderInfo;
}
