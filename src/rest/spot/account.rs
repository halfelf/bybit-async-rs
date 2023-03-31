use crate::models::{AccountInformation, OrderInfo, OrderSide, OrderType, Product, TimeInForce};
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => GetAccount;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/account";
    Signed => true;
    Request => {};
    Response => AccountInformation;
}

crate::define_request! {
    Name => Order;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/order";
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
