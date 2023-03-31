use crate::models::Product;
use reqwest::Method;

crate::define_request! {
    Name => GetCurrentPositionMode;
    Product => Product::UsdMFutures;
    Method => Method::GET;
    Endpoint => "/fapi/v1/positionSide/dual";
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}
