use crate::model::Product;
use reqwest::Method;

crate::define_request! {
    Name => GetCurrentPositionModeUsdM;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/positionSide/dual";
    Method => Method::GET;
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}
