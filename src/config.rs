#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub usdm_futures_rest_api_endpoint: String,
    pub usdm_futures_ws_endpoint: String,

    pub coinm_futures_rest_api_endpoint: String,
    pub coinm_futures_ws_endpoint: String,

    pub european_options_rest_api_endpoint: String,
    pub european_options_ws_endpoint: String,

    pub recv_window: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443/ws".into(),

            usdm_futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            usdm_futures_ws_endpoint: "wss://fstream.binance.com/ws".into(),

            coinm_futures_rest_api_endpoint: "https://dapi.binance.com".into(),
            coinm_futures_ws_endpoint: "wss://dstream.binance.com/ws".into(),

            european_options_rest_api_endpoint: "https://eapi.binance.com".into(),
            european_options_ws_endpoint: "wss://estream.binance.com/ws".into(),

            recv_window: 5000,
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        // Self::default()
        //     .set_rest_api_endpoint("https://testnet.binance.vision")
        //     .set_ws_endpoint("wss://testnet.binance.vision/ws")
        //     .set_futures_rest_api_endpoint("https://testnet.binancefuture.com")
        //     .set_futures_ws_endpoint("https://testnet.binancefuture.com/ws")
        todo!()
    }
}
