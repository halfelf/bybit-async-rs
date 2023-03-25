use super::APIUrl;
use crate::model::{AccountInformation, OrderInfo, OrderSide, OrderType, TimeInForce};
use reqwest::Method;
use rust_decimal::Decimal;

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
        pub symbol: String,
        pub qty: Decimal,
        pub price: Option<Decimal>,
        pub stop_price: Option<Decimal>,
        pub order_side: OrderSide,
        pub order_type: OrderType,
        pub time_in_force: TimeInForce,
        pub new_client_order_id: Option<String>,
    };
    Response => OrderInfo;
}
