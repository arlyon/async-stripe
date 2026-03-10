#![recursion_limit = "256"]
#![deny(clippy::large_stack_frames)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(non_camel_case_types)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! centered around [Reserve](https://docs.stripe.com/api/reserves)

extern crate self as stripe_reserve;

miniserde::make_place!(Place);
#[doc(hidden)]
pub mod reserve_hold;
#[doc(inline)]
pub use reserve_hold::*;
#[doc(hidden)]
pub mod reserve_plan;
#[doc(inline)]
pub use reserve_plan::*;
#[doc(hidden)]
pub mod reserve_release;
#[doc(inline)]
pub use reserve_release::*;
#[doc(hidden)]
pub mod reserves_reserve_holds_resources_release_schedule;
#[doc(inline)]
pub use reserves_reserve_holds_resources_release_schedule::*;
#[doc(hidden)]
pub mod reserves_reserve_plans_resources_fixed_release;
#[doc(inline)]
pub use reserves_reserve_plans_resources_fixed_release::*;
#[doc(hidden)]
pub mod reserves_reserve_plans_resources_rolling_release;
#[doc(inline)]
pub use reserves_reserve_plans_resources_rolling_release::*;
#[doc(hidden)]
pub mod reserves_reserve_releases_resources_source_transaction;
#[doc(inline)]
pub use reserves_reserve_releases_resources_source_transaction::*;
