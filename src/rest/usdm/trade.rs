use crate::models::{
    CanceledOrder, NewOrderResponseType, OrderSide, OrderType, PositionSide, Product, TimeInForce,
    WorkingType,
};
use crate::parser::{string_or_decimal, string_or_decimal_opt};
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => NewOrder;
    Product => Product::UsdMFutures;
    Method => Method::POST;
    Endpoint => "/fapi/v1/order";
    Signed => true;
    Request => {
        pub symbol: String,
        pub side: OrderSide,
        pub position_side: Option<PositionSide>,
        pub r#type: OrderType,
        pub time_in_force: Option<TimeInForce>,
        pub quantity: Option<Decimal>,
        pub reduce_only: Option<bool>,
        pub price: Option<Decimal>,
        pub new_client_order_id: Option<String>,
        pub stop_price: Option<Decimal>,
        pub close_position: Option<bool>,
        pub activation_price: Option<Decimal>,
        pub callback_rate: Option<Decimal>,
        pub working_type: Option<WorkingType>,
        pub price_protect: Option<Decimal>,
        pub new_order_resp_type: Option<NewOrderResponseType>,
    };
    Response => {
        pub client_order_id: String,
        #[serde(with = "string_or_decimal")]
        pub cum_qty: Decimal,
        #[serde(with = "string_or_decimal")]
        pub cum_quote: Decimal,
        #[serde(with = "string_or_decimal")]
        pub executed_qty: Decimal,
        pub order_id: u64,
        #[serde(with = "string_or_decimal")]
        pub avg_price: Decimal,
        #[serde(with = "string_or_decimal")]
        pub orig_qty: Decimal,
        pub reduce_only: bool,
        pub side: String,
        pub position_side: String,
        pub status: String,
        #[serde(with = "string_or_decimal")]
        pub stop_price: Decimal,
        pub close_position: bool,
        pub symbol: String,
        pub time_in_force: String,
        #[serde(rename = "type")]
        pub type_name: String,
        pub orig_type: String,
        #[serde(default)]
        #[serde(with = "string_or_decimal_opt")]
        pub activate_price: Option<Decimal>,
        #[serde(default)]
        #[serde(with = "string_or_decimal_opt")]
        pub price_rate: Option<Decimal>,
        pub update_time: u64,
        pub working_type: String,
        price_protect: bool,
    };
}

crate::define_request! {
    Name => CancelOrder;
    Product => Product::UsdMFutures;
    Method => Method::DELETE;
    Endpoint => "/fapi/v1/order";
    Signed => true;
    Request => {
        pub symbol: String,
        pub order_id: Option<u64>,
        pub orig_client_order_id: Option<String>
    };
    Response => CanceledOrder;
}

crate::define_request! {
    Name => CancelMultipleOrders;
    Product => Product::UsdMFutures;
    Method => Method::DELETE;
    Endpoint => "/fapi/v1/batchOrders";
    Signed => true;
    Request => {
        pub symbol: String,
        pub order_id_list: Vec<u64>,
        pub orig_client_order_id_list: Vec<String>,
    };
    Response => Vec<CanceledOrder>;
}
