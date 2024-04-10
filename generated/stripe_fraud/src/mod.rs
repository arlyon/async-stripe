#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Fraud` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_fraud;

miniserde::make_place!(Place);
#[doc(hidden)]
pub mod deleted_radar_value_list;
#[doc(inline)]
pub use deleted_radar_value_list::*;
#[doc(hidden)]
pub mod deleted_radar_value_list_item;
#[doc(inline)]
pub use deleted_radar_value_list_item::*;
pub use radar_early_fraud_warning::types::*;
pub mod radar_early_fraud_warning;
pub use radar_value_list::types::*;
pub mod radar_value_list;
pub use radar_value_list_item::types::*;
pub mod radar_value_list_item;
pub use stripe_shared::radar_review_resource_location::*;
pub use stripe_shared::radar_review_resource_session::*;
pub mod review;
pub use stripe_shared::review::*;
pub use stripe_shared::rule::*;
