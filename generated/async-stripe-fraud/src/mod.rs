#![recursion_limit = "256"]
#![deny(clippy::large_stack_frames)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(non_camel_case_types)]

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
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_address;
#[doc(inline)]
pub use insights_resources_payment_evaluation_address::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_billing_details;
#[doc(inline)]
pub use insights_resources_payment_evaluation_billing_details::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_card_issuer_decline;
#[doc(inline)]
pub use insights_resources_payment_evaluation_card_issuer_decline::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_client_device_metadata;
#[doc(inline)]
pub use insights_resources_payment_evaluation_client_device_metadata::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_customer_details;
#[doc(inline)]
pub use insights_resources_payment_evaluation_customer_details::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_dispute_opened;
#[doc(inline)]
pub use insights_resources_payment_evaluation_dispute_opened::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_early_fraud_warning_received;
#[doc(inline)]
pub use insights_resources_payment_evaluation_early_fraud_warning_received::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_event;
#[doc(inline)]
pub use insights_resources_payment_evaluation_event::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_insights;
#[doc(inline)]
pub use insights_resources_payment_evaluation_insights::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_merchant_blocked;
#[doc(inline)]
pub use insights_resources_payment_evaluation_merchant_blocked::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_money_movement_card;
#[doc(inline)]
pub use insights_resources_payment_evaluation_money_movement_card::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_money_movement_details;
#[doc(inline)]
pub use insights_resources_payment_evaluation_money_movement_details::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_outcome;
#[doc(inline)]
pub use insights_resources_payment_evaluation_outcome::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_payment_details;
#[doc(inline)]
pub use insights_resources_payment_evaluation_payment_details::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_payment_method_details;
#[doc(inline)]
pub use insights_resources_payment_evaluation_payment_method_details::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_refunded;
#[doc(inline)]
pub use insights_resources_payment_evaluation_refunded::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_rejected;
#[doc(inline)]
pub use insights_resources_payment_evaluation_rejected::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_rejected_card;
#[doc(inline)]
pub use insights_resources_payment_evaluation_rejected_card::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_scorer;
#[doc(inline)]
pub use insights_resources_payment_evaluation_scorer::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_shipping;
#[doc(inline)]
pub use insights_resources_payment_evaluation_shipping::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_succeeded;
#[doc(inline)]
pub use insights_resources_payment_evaluation_succeeded::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_succeeded_card;
#[doc(inline)]
pub use insights_resources_payment_evaluation_succeeded_card::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_user_intervention_raised;
#[doc(inline)]
pub use insights_resources_payment_evaluation_user_intervention_raised::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_user_intervention_raised_custom;
#[doc(inline)]
pub use insights_resources_payment_evaluation_user_intervention_raised_custom::*;
#[doc(hidden)]
pub mod insights_resources_payment_evaluation_user_intervention_resolved;
#[doc(inline)]
pub use insights_resources_payment_evaluation_user_intervention_resolved::*;
pub use radar_early_fraud_warning::types::*;
pub mod radar_early_fraud_warning;
pub use radar_payment_evaluation::types::*;
pub mod radar_payment_evaluation;
pub use radar_value_list::types::*;
pub mod radar_value_list;
pub use radar_value_list_item::types::*;
pub mod radar_value_list_item;
pub use stripe_shared::radar_review_resource_location::*;
pub use stripe_shared::radar_review_resource_session::*;
pub mod review;
pub use stripe_shared::review::*;
pub use stripe_shared::rule::*;
