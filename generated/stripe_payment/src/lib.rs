#![recursion_limit = "256"]
extern crate self as stripe_payment;
pub mod payment_link;
pub use payment_link::PaymentLink;
