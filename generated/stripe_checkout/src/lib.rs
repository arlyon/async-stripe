#![recursion_limit = "256"]
extern crate self as stripe_checkout;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod checkout;
