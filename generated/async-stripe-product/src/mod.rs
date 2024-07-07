#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Products` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_product;

miniserde::make_place!(Place);
pub mod coupon;
pub use stripe_shared::coupon::*;
pub use stripe_shared::coupon_applies_to::*;
pub use stripe_shared::coupon_currency_option::*;
pub use stripe_shared::currency_option::*;
pub use stripe_shared::custom_unit_amount::*;
pub use stripe_shared::deleted_coupon::*;
pub use stripe_shared::deleted_discount::*;
pub use stripe_shared::deleted_product::*;
#[doc(hidden)]
pub mod deleted_product_feature;
#[doc(inline)]
pub use deleted_product_feature::*;
pub use stripe_shared::discount::*;
pub use stripe_shared::line_items_discount_amount::*;
pub use stripe_shared::line_items_tax_amount::*;
pub use stripe_shared::package_dimensions::*;
pub mod price;
pub use stripe_shared::price::*;
pub use stripe_shared::price_tier::*;
pub mod product;
pub use product_feature::types::*;
pub use stripe_shared::product::*;
pub mod product_feature;
pub use stripe_shared::product_marketing_feature::*;
pub mod promotion_code;
pub use stripe_shared::promotion_code::*;
pub use stripe_shared::promotion_code_currency_option::*;
pub use stripe_shared::promotion_codes_resource_restrictions::*;
pub use stripe_shared::recurring::*;
pub mod shipping_rate;
pub use stripe_shared::shipping_rate::*;
pub mod tax_code;
pub use stripe_shared::tax_code::*;
pub mod tax_rate;
pub use stripe_shared::tax_rate::*;
pub use stripe_shared::transform_quantity::*;
