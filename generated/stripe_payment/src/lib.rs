#![recursion_limit = "256"]
extern crate self as stripe_payment;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod payment_link;
pub use payment_link::PaymentLink;
