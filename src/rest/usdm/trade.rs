use crate::model::{
    CanceledOrder, NewOrderResponseType, OrderSide, OrderType, PositionSide, Product, TimeInForce,
    WorkingType,
};
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => NewOrder;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/order";
    Method => Method::POST;
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
    Response => CanceledOrder;
}

crate::define_request! {
    Name => CancelOrder;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/order";
    Method => Method::DELETE;
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
    Endpoint => "/fapi/v1/batchOrders";
    Method => Method::DELETE;
    Signed => true;
    Request => {
        pub symbol: String,
        pub order_id_list: Vec<u64>,
        pub orig_client_order_id_list: Vec<String>,
    };
    Response => Vec<CanceledOrder>;
}
