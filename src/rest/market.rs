use crate::model::Product;

use reqwest::Method;

crate::define_request! {
    Name => Ping;
    Product => Product::Spot;
    Endpoint => "/api/v3/ping";
    Method => Method::GET;
    Signed => false;
    Request => {};
    Response => {};
}
