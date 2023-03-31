use crate::models::{NewOrderResponse, OrderSide, Product};
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => ModifyOrder;
    Product => Product::UsdMFutures;
    Method => Method::PUT;
    Endpoint => "/fapi/v1/order";
    Signed => true;
    Request => {
        pub order_id: Option<u64>,
        pub orig_client_order_id: Option<String>,

        pub symbol: String,
        pub side: OrderSide,
        pub quantity: Option<Decimal>,
        pub price: Option<Decimal>,
    };
    Response => NewOrderResponse;
}
