use super::APIUrl;
use crate::model::{AccountInformation, OrderInfo, OrderSide, OrderType, TimeInForce};
use reqwest::Method;

crate::define_request! {
    Name => GetAccount;
    API => APIUrl::Spot;
    Endpoint => "/api/v3/account";
    Method => Method::GET;
    Signed => true;
    Request => {};
    Response => AccountInformation;
}

crate::define_request! {
    Name => Order;
    API => APIUrl::Spot;
    Endpoint => "/api/v3/order";
    Method => Method::GET;
    Signed => true;
    Request => {
        symbol: String,
        qty: f64,
        price: f64,
        stop_price: Option<f64>,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
        new_client_order_id: Option<String>,
    };
    Response => OrderInfo;
}
