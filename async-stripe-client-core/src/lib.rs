//! This crate provides shared utilities for implementing clients
//! capable of making Stripe API requests.

#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]
mod config;
mod pagination;
mod request_strategy;
mod stripe_request;

pub use config::{ConfigOverride, SharedConfigBuilder};
pub use pagination::*;
pub use request_strategy::*;
pub use stripe_request::*;
pub use stripe_shared::version::VERSION;
pub use stripe_shared::{AccountId, ApiVersion, ApplicationId};
