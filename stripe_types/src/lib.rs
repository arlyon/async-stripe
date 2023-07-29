#![recursion_limit = "128"]

pub mod currency;
pub mod deser;
pub mod generated;
pub mod ids;
mod pagination;
pub mod params;

pub use currency::Currency;
pub use deser::StripeDeserialize;
pub use generated::*;
pub use ids::*;
pub use pagination::*;
pub use params::*;

// Allow generated code to use absolute paths starting with `stripe` instead of `crate`
extern crate self as stripe_types;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
