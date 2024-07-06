#![allow(clippy::large_enum_variant)]
#![recursion_limit = "256"]
//! This crate provides Rust bindings for handling [Stripe webhook events](https://stripe.com/docs/webhooks).
//!
//! Please see the associated examples for basic usage with
//! [axum](https://github.com/tokio-rs/axum), [actix-web](https://github.com/actix/actix-web),
//! and [rocket](https://rocket.rs/)
mod error;
mod generated;
mod webhook;

pub use error::WebhookError;
pub use generated::*;
pub use stripe_shared::event::EventType;
pub use webhook::{Event, Webhook};
