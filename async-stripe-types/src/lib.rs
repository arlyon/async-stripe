//! This crate provides Rust bindings for core types to the Stripe HTTP API.
#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]

mod currency;
mod error;
mod expandable;
mod ids;
mod pagination;
mod params;

pub use currency::Currency;
pub use error::StripeParseError;
pub use expandable::*;
pub use pagination::*;
pub use params::*;
pub use serde_helpers::{with_serde_json, with_serde_json_opt};

miniserde::make_place!(Place);

#[doc(hidden)]
pub mod miniserde_helpers;

#[doc(hidden)]
pub mod serde_helpers;

// Allow generated code to use absolute paths starting with `stripe_types` instead of `crate`
extern crate self as stripe_types;
