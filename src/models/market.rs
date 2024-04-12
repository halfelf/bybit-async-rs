use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    interval: Interval,
    interval_num: usize,
    limit: usize,
    rate_limit_type: RateLimitType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Interval {
    Minute,
}

impl Default for Interval {
    fn default() -> Self {
        Interval::Minute
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
}

impl Default for RateLimitType {
    fn default() -> Self {
        RateLimitType::RequestWeight
    }
}
