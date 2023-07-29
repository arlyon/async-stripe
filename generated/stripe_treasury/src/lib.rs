#![recursion_limit = "256"]
extern crate self as stripe_treasury;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod treasury;
