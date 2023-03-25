use super::super::Request;
use crate::rest::APIUrl;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelMultipleOrdersRequest {
    symbol: String,
    order_id_list: Vec<u64>,
    orig_client_order_id_list: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelMultipleOrdersResponse {
    dual_side_position: bool,
}

impl Request for CancelMultipleOrdersRequest {
    const API: APIUrl = APIUrl::UsdMFutures;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    type Response = CancelMultipleOrdersResponse;
}
