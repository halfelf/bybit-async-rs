use crate::models::{
    spot::{CancelOrderResponse, CanceledOrder, NewOrderResponse, NewOrderResponseType},
    usdm::PositionSide,
    usdm::WorkingType,
    OrderType, Product, Side, TimeInForce,
};
use fehler::throw;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};

crate::define_request! {
    Name => NewOrder;
    Product => Product::UsdMFutures;
    Method => Method::POST;
    Endpoint => "/fapi/v1/order";
    Signed => true;
    Request => {
        pub symbol: String,
        pub side: Side,
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
    Response => NewOrderResponse;
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
    Response => CancelOrderResponse;
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

crate::define_request! {
    Name => CancelAllOpenOrders;
    Product => Product::UsdMFutures;
    Method => Method::DELETE;
    Endpoint => "/fapi/v1/allOpenOrders";
    Signed => true;
    Request => {
        pub symbol: String,
    };
    Response => CancelAllOpenOrdersResponse;
}

#[derive(Deserialize, Debug, Clone)]
pub struct CancelAllOpenOrdersResponse {
    #[serde(deserialize_with = "check")]
    pub code: u64,
    pub msg: String,
}

pub fn check<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u64::deserialize(d)?;
    if v != 200 {
        throw!(serde::de::Error::custom("not success code"))
    }
    Ok(v)
}
