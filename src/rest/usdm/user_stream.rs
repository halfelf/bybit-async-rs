use crate::rest::Product;
use reqwest::Method;

crate::define_request! {
    Name => StartUserDataStream;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/listenKey";
    Method => Method::POST;
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {
        pub listen_key: String,
    };
}

crate::define_request! {
    Name => KeepaliveUserDataStream;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/listenKey";
    Method => Method::PUT;
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}

crate::define_request! {
    Name => CloseUserDataStream;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/listenKey";
    Method => Method::DELETE;
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}
