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
            rest_api_endpoint: "https://api.bybit.com".into(),
            ws_endpoint: "wss://stream.bybit.com:9443".into(),

            usdm_futures_rest_api_endpoint: "https://fapi.bybit.com".into(),
            usdm_futures_ws_endpoint: "wss://fstream.bybit.com".into(),

            coinm_futures_rest_api_endpoint: "https://dapi.bybit.com".into(),
            coinm_futures_ws_endpoint: "wss://dstream.bybit.com".into(),

            european_options_rest_api_endpoint: "https://eapi.bybit.com".into(),
            european_options_ws_endpoint: "wss://estream.bybit.com".into(),

            recv_window: 5000,
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        // Self::default()
        //     .set_rest_api_endpoint("https://testnet.bybit.vision")
        //     .set_ws_endpoint("wss://testnet.bybit.vision/ws")
        //     .set_futures_rest_api_endpoint("https://testnet.bybitfuture.com")
        //     .set_futures_ws_endpoint("https://testnet.bybitfuture.com/ws")
        todo!()
    }
}
