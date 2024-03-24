//! This crate provides Rust bindings for core types to the Stripe HTTP API.

mod currency;
mod deser;
mod expandable;
mod ids;
mod pagination;
mod params;

pub use currency::{Currency, ParseCurrencyError};
pub use deser::{StripeDeserialize, Value};
pub use expandable::*;
pub use ids::*;
pub use pagination::*;
pub use params::*;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
#[cfg(feature = "min-ser")]
#[doc(hidden)]
pub mod miniserde_helpers;

// Allow generated code to use absolute paths starting with `stripe` instead of `crate`
extern crate self as stripe_types;
