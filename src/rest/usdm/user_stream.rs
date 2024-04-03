use crate::rest::Product;
use reqwest::Method;

crate::define_request! {
    Name => KeepaliveUserDataStream;
    Product => Product::UsdMFutures;
    Method => Method::PUT;
    Endpoint => "/fapi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}

crate::define_request! {
    Name => CloseUserDataStream;
    Product => Product::UsdMFutures;
    Method => Method::DELETE;
    Endpoint => "/fapi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}
