use crate::models::Product;

const REST_API_ENDPOINT: &str = "https://api.bybit.com";
const SPOT_WS_ENDPOINT: &str = "wss://stream.bybit.com/v5/public/spot";
const USDM_FUTURES_WS_ENDPOINT: &str = "wss://stream.bybit.com/v5/public/linear";
const COINM_FUTURES_WS_ENDPOINT: &str = "wss://stream.bybit.com/v5/public/inverse";
const EUROPEAN_OPTIONS_WS_ENDPOINT: &str = "wss://stream.bybit.com/v5/public/option";
const PRIVATE_WS_ENDPOINT: &str = "wss://stream.bybit.com/v5/private";

#[derive(Clone, Debug)]
pub struct Config {
    pub product: Product,

    pub rest_api_endpoint: String,  /// Unified REST API endpoint

    pub spot_ws_endpoint: String,
    pub usdm_futures_ws_endpoint: String,
    pub coinm_futures_ws_endpoint: String,
    pub european_options_ws_endpoint: String,
    pub private_ws_endpoint: String,

    pub recv_window: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            product: Product::Spot,

            rest_api_endpoint: REST_API_ENDPOINT.into(),
            spot_ws_endpoint: SPOT_WS_ENDPOINT.into(),
            usdm_futures_ws_endpoint: USDM_FUTURES_WS_ENDPOINT.into(),
            coinm_futures_ws_endpoint: COINM_FUTURES_WS_ENDPOINT.into(),
            european_options_ws_endpoint: EUROPEAN_OPTIONS_WS_ENDPOINT.into(),
            private_ws_endpoint: PRIVATE_WS_ENDPOINT.into(),
            recv_window: 5000,
        }
    }
}

impl Config {
    pub fn new(product: Product) -> Self {
        Self {
            product,
            ..Self::default()
        }
    }

    pub fn testnet() -> Self {
        // Self::default()
        //     .set_rest_api_endpoint("https://testnet.bybit.vision")
        //     .set_ws_endpoint("wss://testnet.bybit.vision/ws")
        //     .set_futures_rest_api_endpoint("https://testnet.bybitfuture.com")
        //     .set_futures_ws_endpoint("https://testnet.bybitfuture.com/ws")
        todo!()
    }
}
