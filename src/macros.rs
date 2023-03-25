#[macro_export]
macro_rules! define_request {
    (
        Name => $name: ident;
        API => $api: expr;
        Endpoint => $endpoint: expr;
        Method => $method: expr;
        Signed => $signed: expr;
        Request => { $($req_field: ident : $req_type: ty,)* };
        Response => { $($resp_field: ident : $resp_type: ty,)* };
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Request>] {
                $(pub $req_field: $req_type),*
            }

            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct [<$name Response>] {
                $(pub $resp_field: $resp_type),*
            }

            impl crate::rest::Request for [<$name Request>] {
                const API: crate::rest::APIUrl = $api;
                const ENDPOINT: &'static str = $endpoint;
                const METHOD: reqwest::Method = $method;
                const SIGNED: bool = $signed;
                type Response = [<$name Response>];
            }
        }
    };

    (
        Name => $name: ident;
        API => $api: expr;
        Endpoint => $endpoint: expr;
        Method => $method: expr;
        Signed => $signed: expr;
        Request => { $($req_field: ident : $req_type: ty,)* };
        Response => $resp_ty: ty;
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            pub struct [<$name Request>] {
                $(pub $req_field: $req_type),*
            }

            impl crate::rest::Request for [<$name Request>] {
                const API: crate::rest::APIUrl = $api;
                const ENDPOINT: &'static str = $endpoint;
                const METHOD: reqwest::Method = $method;
                const SIGNED: bool = $signed;
                type Response = $resp_ty;
            }
        }
    };
}
