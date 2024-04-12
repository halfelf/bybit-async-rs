use crate::models::usdm::{Asset, Position};
use crate::models::Product;
use crate::parser::string_or;
use crate::parser::string_or_decimal;
use reqwest::Method;
use rust_decimal::Decimal;

crate::define_request! {
    Name => GetCurrentPositionMode;
    Product => Product::UsdMFutures;
    Method => Method::GET;
    Endpoint => "/fapi/v1/positionSide/dual";
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}

crate::define_request! {
    Name => AccountInformationV2;
    Product => Product::UsdMFutures;
    Method => Method::GET;
    Endpoint => "/fapi/v2/account";
    Signed => true;
    Request => {};
    Response => {
        #[serde(with = "string_or_decimal")]
        pub fee_tier: Decimal,
        #[serde(with = "string_or")]
        pub can_trade: bool,
        #[serde(with = "string_or")]
        pub can_deposit: bool,
        #[serde(with = "string_or")]
        pub can_withdraw: bool,
        #[serde(with = "string_or_decimal")]
        pub update_time: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_initial_margin: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_maint_margin: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_wallet_balance: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_unrealized_profit: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_margin_balance: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_position_initial_margin: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_open_order_initial_margin: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_cross_wallet_balance: Decimal,
        #[serde(with = "string_or_decimal")]
        pub total_cross_un_pnl: Decimal,
        #[serde(with = "string_or_decimal")]
        pub available_balance: Decimal,
        #[serde(with = "string_or_decimal")]
        pub max_withdraw_amount: Decimal,
        pub assets: Vec<Asset>,
        pub positions: Vec<Position>,
    };
}
