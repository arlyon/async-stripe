//! This crate provides Rust bindings for core types to the Stripe HTTP API.

mod currency;
mod expandable;
mod ids;
mod pagination;
mod params;

pub use currency::{Currency, ParseCurrencyError};
pub use expandable::*;
pub use pagination::*;
pub use params::*;
pub use serde_helpers::{with_serde_json, with_serde_json_opt};

miniserde::make_place!(Place);

#[doc(hidden)]
pub mod miniserde_helpers;

#[doc(hidden)]
pub mod serde_helpers;

// Allow generated code to use absolute paths starting with `stripe` instead of `crate`
extern crate self as stripe_types;
