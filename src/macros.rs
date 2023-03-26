#[macro_export]
macro_rules! define_request {
    (
        Name => $name: ident;
        Product => $product: expr;
        Endpoint => $endpoint: expr;
        Method => $method: expr;
        Signed => $signed: expr;
        Request => { $($req_def:tt)* };
        Response => { $($resp_def:tt)* };
    ) => {
        crate::define_request! {
            Name => $name;
            Product => $product;
            Endpoint => $endpoint;
            Method => $method;
            Keyed => false;
            Signed => $signed;
            Request => { $($req_def)* };
            Response => { $($resp_def)* };
        }
    };
    (
        Name => $name: ident;
        Product => $product: expr;
        Endpoint => $endpoint: expr;
        Method => $method: expr;
        Keyed => $keyed: expr;
        Signed => $signed: expr;
        Request => { $($req_def:tt)* };
        Response => { $($resp_def:tt)* };
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Request>] {
                $($req_def)*
            }

            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct [<$name Response>] {
                $($resp_def)*
            }

            impl crate::rest::Request for [<$name Request>] {
                const PRODUCT: crate::rest::Product = $product;
                const ENDPOINT: &'static str = $endpoint;
                const METHOD: reqwest::Method = $method;
                const KEYED: bool = $keyed;
                const SIGNED: bool = $signed;
                type Response = [<$name Response>];
            }
        }
    };

    (
        Name => $name: ident;
        Product => $product: expr;
        Endpoint => $endpoint: expr;
        Method => $method: expr;
        Signed => $signed: expr;
        Request => { $($req_def:tt)* };
        Response => $resp_ty: ty;
    ) => {
        crate::define_request! {
            Name => $name;
            Product => $product;
            Endpoint => $endpoint;
            Method => $method;
            Keyed => false;
            Signed => $signed;
            Request => { $($req_def)* };
            Response => $resp_ty;
        }
    };
    (
        Name => $name: ident;
        Product => $product: expr;
        Endpoint => $endpoint: expr;
        Method => $method: expr;
        Keyed => $keyed: expr;
        Signed => $signed: expr;
        Request => { $($req_def:tt)* };
        Response => $resp_ty: ty;
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Request>] {
                $($req_def)*
            }

            impl crate::rest::Request for [<$name Request>] {
                const PRODUCT: crate::rest::Product = $product;
                const ENDPOINT: &'static str = $endpoint;
                const METHOD: reqwest::Method = $method;
                const KEYED: bool = $keyed;
                const SIGNED: bool = $signed;
                type Response = $resp_ty;
            }
        }
    };
}
