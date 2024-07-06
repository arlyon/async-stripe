//! This crate provides Rust bindings to the Stripe HTTP API, using the
//! `async-std` runtime and a `surf` HTTP client.
//!
//! See the `async-stripe` crate for the usage guide.
#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]

mod client;
mod config;
mod error;

pub use client::Client;
pub use config::ClientBuilder;
pub use error::StripeError;
