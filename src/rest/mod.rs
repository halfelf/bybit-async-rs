pub mod coinm;
pub mod spot;
pub mod usdm;

use crate::{
    config::Config,
    error::BybitError::{self, *},
    models::Product,
    BybitResponseError,
};
use chrono::Utc;
use fehler::{throw, throws};
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use log::{debug, trace};
#[cfg(feature = "zero-copy")]
use owning_ref::OwningHandle;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Client, Method, Response,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;
use sha2::Sha256;
#[cfg(feature = "zero-copy")]
use std::ops::Deref;

pub trait Request: Serialize {
    const PRODUCT: Product;
    const ENDPOINT: &'static str;
    const METHOD: Method;
    const KEYED: bool = false; // SIGNED imples KEYED no matter KEYED is true or false
    const SIGNED: bool = false;
    type Response: DeserializeOwned;
}

#[derive(Clone, Default)]
pub struct Bybit {
    key: Option<String>,
    secret: Option<String>,
    client: Client,
    config: Config,
}

impl Bybit {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_key(api_key: &str) -> Self {
        Bybit {
            client: Client::new(),
            key: Some(api_key.into()),
            secret: None,
            config: Config::default(),
        }
    }

    pub fn with_key_and_secret(api_key: &str, api_secret: &str) -> Self {
        Bybit {
            client: Client::new(),
            key: Some(api_key.into()),
            secret: Some(api_secret.into()),
            config: Config::default(),
        }
    }

    pub fn config(&mut self, config: Config) {
        self.config = config;
    }

    #[throws(BybitError)]
    pub async fn request<R>(&self, req: R) -> RestResponse<R::Response>
    where
        R: Request,
    {
        let mut params = if matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req)?
        } else {
            String::new()
        };

        let body = if !matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req)?
        } else {
            String::new()
        };

        if R::SIGNED {
            if !params.is_empty() {
                params.push('&');
            }
            params.push_str(&format!("timestamp={}", Utc::now().timestamp_millis()));
            params.push_str(&format!("&recvWindow={}", self.config.recv_window));

            let signature = self.signature(&params, &body)?;
            params.push_str(&format!("&signature={}", signature));
        }

        let path = R::ENDPOINT.to_string();

        let base = match R::PRODUCT {
            Product::Spot => &self.config.rest_api_endpoint,
            Product::UsdMFutures => &self.config.usdm_futures_rest_api_endpoint,
            Product::CoinMFutures => &self.config.coinm_futures_rest_api_endpoint,
            Product::EuropeanOptions => &self.config.european_options_rest_api_endpoint,
        };
        let url = format!("{base}{path}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("bybit-async-rs"));
        if !body.is_empty() {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        if R::SIGNED || R::KEYED {
            let key = match &self.key {
                Some(key) => key,
                None => throw!(MissingApiKey),
            };
            custom_headers.insert(
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(key)?,
            );
        }

        debug!("[REST] url: {url}, body: {body}");

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .body(body)
            .send()
            .await?;

        self.handle_response(resp).await?
    }

    #[throws(BybitError)]
    fn signature(&self, params: &str, body: &str) -> String {
        let secret = match &self.secret {
            Some(s) => s,
            None => throw!(MissingApiSecret),
        };
        // Signature: hex(HMAC_SHA256(queries + data))
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        let sign_message = format!("{}{}", params, body);
        trace!("Sign message: {}", sign_message);
        mac.update(sign_message.as_bytes());
        let signature = hexify(mac.finalize().into_bytes());
        signature
    }

    #[cfg(not(feature = "zero-copy"))]
    #[throws(BybitError)]
    async fn handle_response<O: DeserializeOwned>(&self, resp: Response) -> RestResponse<O> {
        let status = resp.status();
        let body = resp.text().await?;

        if cfg!(feature = "print-response") {
            debug!("Response is {status} {body}");
        };

        match from_str(&body) {
            Ok(v) => v,
            Err(e) => match from_str::<BybitResponseError>(&body) {
                Ok(e) => throw!(e),
                Err(_) => throw!(e),
            },
        }
    }

    #[cfg(feature = "zero-copy")]
    #[throws(BybitError)]
    async fn handle_response<O: DeserializeOwned>(&self, resp: Response) -> RestResponse<O> {
        let status = resp.status();
        let body = resp.text().await?;

        if cfg!(feature = "print-response") {
            debug!("Response is {status} {body}");
        };

        OwningHandle::try_new(body, |body| -> Result<_, BybitError> {
            let body = unsafe { &*body };
            match from_str(body) {
                Ok(v) => Ok(C(v)),
                Err(e) => match from_str::<BybitResponseError>(&body) {
                    Ok(e) => throw!(e),
                    Err(_) => throw!(e),
                },
            }
        })?
    }
}

#[cfg(feature = "zero-copy")]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct C<T>(pub T);
#[cfg(feature = "zero-copy")]
impl<T> Deref for C<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "zero-copy")]
pub type RestResponse<O> = OwningHandle<String, C<O>>;

#[cfg(not(feature = "zero-copy"))]
pub type RestResponse<O> = O;

#[cfg(test)]
mod test {
    use super::Bybit;
    use anyhow::Error;
    use fehler::throws;
    use url::{form_urlencoded::Serializer, Url};

    #[throws(Error)]
    #[test]
    fn signature_query() {
        let tr = Bybit::with_key_and_secret(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let sig = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/test",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                    ("quantity", "1"),
                    ("price", "0.1"),
                    ("recvWindow", "5000"),
                    ("timestamp", "1499827319559"),
                ],
            )?
            .query()
            .unwrap_or_default(),
            "",
        )?;
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }

    #[throws(Error)]
    #[test]
    fn signature_body() {
        let tr = Bybit::with_key_and_secret(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[
            ("symbol", "LTCBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ]);

        let sig = tr.signature(
            &Url::parse("http://a.com/api/v1/test")?
                .query()
                .unwrap_or_default(),
            &s.finish(),
        )?;
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }

    #[throws(Error)]
    #[test]
    fn signature_query_body() {
        let tr = Bybit::with_key_and_secret(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );

        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ]);

        let sig = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/order",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                ],
            )?
            .query()
            .unwrap_or_default(),
            &s.finish(),
        )?;
        assert_eq!(
            sig,
            "0fd168b8ddb4876a0358a8d14d0c9f3da0e9b20c5d52b2a00fcf7d1c602f9a77"
        );
    }

    #[throws(Error)]
    #[test]
    fn signature_body2() {
        let tr = Bybit::with_key_and_secret(
            "vj1e6h50pFN9CsXT5nsL25JkTuBHkKw3zJhsA6OPtruIRalm20vTuXqF3htCZeWW",
            "5Cjj09rLKWNVe7fSalqgpilh5I3y6pPplhOukZChkusLqqi9mQyFk34kJJBTdlEJ",
        );

        let q = &mut [
            ("symbol", "ETHBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1540687064555"),
        ];
        q.sort();
        let q: Vec<_> = q.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        let q = q.join("&");
        let sig = tr.signature(
            &Url::parse("http://a.com/api/v1/test")?
                .query()
                .unwrap_or_default(),
            &q,
        )?;
        assert_eq!(
            sig,
            "1ee5a75760b9496a2144a22116e02bc0b7fdcf828781fa87ca273540dfcf2cb0"
        );
    }
}
