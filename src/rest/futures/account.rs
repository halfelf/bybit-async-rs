use super::super::Request;
use crate::rest::APIUrl;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GetCurrentPositionModeRequest {}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentPositionModeResponse {
    dual_side_position: bool,
}

impl Request for GetCurrentPositionModeRequest {
    const API: APIUrl = APIUrl::UsdMFutures;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    type Response = GetCurrentPositionModeResponse;
}
