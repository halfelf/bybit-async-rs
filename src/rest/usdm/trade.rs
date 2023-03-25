use crate::rest::APIUrl;
use reqwest::Method;

crate::define_request! {
    Name => CancelMultipleOrders;
    API => APIUrl::UsdMFutures;
    Endpoint => "/fapi/v1/batchOrders";
    Method => Method::DELETE;
    Signed => true;
    Request => {};
    Response => {
        pub symbol: String,
        pub order_id_list: Vec<u64>,
        pub orig_client_order_id_list: Vec<String>,
    };
}
