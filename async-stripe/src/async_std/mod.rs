//! This module provides Rust bindings to the Stripe HTTP API, using the
//! `async-std` runtime and a `surf` HTTP client.
mod client;
mod config;

pub use client::Client;
pub use config::ClientBuilder;
