use crate::models::{spot::NewOrderResponse, Product, Side};
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => ModifyOrder;
    Product => Product::UsdMFutures;
    Method => Method::PUT;
    Endpoint => "/dapi/v1/order";
    Signed => true;
    Request => {
        pub order_id: Option<u64>,
        pub orig_client_order_id: Option<String>,

        pub symbol: String,
        pub side: Side,
        pub quantity: Option<Decimal>,
        pub price: Option<Decimal>,
    };
    Response => NewOrderResponse;
}
