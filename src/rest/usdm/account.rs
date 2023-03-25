use crate::rest::APIUrl;
use reqwest::Method;

crate::define_request! {
    Name => GetCurrentPositionMode;
    API => APIUrl::UsdMFutures;
    Endpoint => "/fapi/v1/positionSide/dual";
    Method => Method::GET;
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}
