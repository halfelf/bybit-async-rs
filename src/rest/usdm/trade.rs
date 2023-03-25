use crate::{
    model::{CanceledOrder, OrderSide, OrderType, PositionSide, TimeInForce, WorkingType},
    rest::APIUrl,
};
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => NewOrder;
    API => APIUrl::UsdMFutures;
    Endpoint => "/fapi/v1/order";
    Method => Method::POST;
    Signed => true;
    Request => {
        pub symbol: String,
        pub side: OrderSide,
        pub position_side: Option<PositionSide>,
        pub order_type: OrderType,
        pub time_in_force: Option<TimeInForce>,
        pub qty: Option<Decimal>,
        pub reduce_only: Option<bool>,
        pub price: Option<Decimal>,
        pub stop_price: Option<Decimal>,
        pub close_position: Option<bool>,
        pub activation_price: Option<Decimal>,
        pub callback_rate: Option<Decimal>,
        pub working_type: Option<WorkingType>,
        pub price_protect: Option<Decimal>,
    };
    Response => CanceledOrder;
}

crate::define_request! {
    Name => CancelOrder;
    API => APIUrl::UsdMFutures;
    Endpoint => "/fapi/v1/batchOrders";
    Method => Method::DELETE;
    Signed => true;
    Request => {
        pub symbol: String,
        pub order_id: u64,
        pub orig_client_order_id: String,
    };
    Response => CanceledOrder;
}

crate::define_request! {
    Name => CancelMultipleOrders;
    API => APIUrl::UsdMFutures;
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
