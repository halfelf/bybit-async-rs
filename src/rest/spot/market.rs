use crate::models::Product;

use reqwest::Method;

crate::define_request! {
    Name => Ping;
    Product => Product::Spot;
    Method => Method::GET;
    Endpoint => "/api/v3/ping";
    Signed => false;
    Request => {};
    Response => {};
}
