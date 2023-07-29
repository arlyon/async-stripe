#![recursion_limit = "256"]
extern crate self as stripe_terminal;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod terminal;
