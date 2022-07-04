// Copyright 2019 Wyyerd Group, LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/async-stripe/")]
#![recursion_limit = "128"]

//! This crate provides Rust bindings to the Stripe HTTP API.
//!
//! ## Getting Started
//!
//! To get started, we need to create a [Client]:
//!
//! ```
//! let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! ```
//!
//! Then we can begin making requests as we'd like.  Most Stripe requests accept
//! many optional parameters, so we usually get the `::new(...)` with any required
//! params and then set the ones we want from there.
//!
//! Most requests for creating or updating a Stripe object use the same Rust struct,
//! so you may frequently need to refer to the [official API docs](https://stripe.com/docs/api)
//! to determine which fields are required for either request.
//!
//! > **Note:** We have an extensive collection of examples which are interspersed in
//!   the documentation. Any time an API is used in an example it is highlighted in the
//!   docs for that item. You can also find all the raw examples in the `examples` directory.
//!   Please have a look at those for inspiration or ideas on how to get started.

#![allow(clippy::map_clone, clippy::large_enum_variant)]
#![warn(clippy::unwrap_used, clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![forbid(unsafe_code)]
#![feature(generic_associated_types)]

// Give a clear error when a required runtime error is not present. Would be better for this
// to be a fatal error preventing emission of further compile errors relating to lack of
// a runtime feature, but that does not seem currently possible:
// https://github.com/rust-lang/rust/issues/68838

#[cfg(not(any(
    feature = "runtime-tokio-hyper",
    feature = "runtime-tokio-hyper-rustls",
    feature = "runtime-blocking",
    feature = "runtime-blocking-rustls",
    feature = "runtime-async-std-surf",
)))]
compile_error!(
    r"one of the following runtime features must be enabled:
    [
        'runtime-tokio-hyper', 
        'runtime-tokio-hyper-rustls',
        'runtime-blocking', 
        'runtime-blocking-rustls', 
        'runtime-async-std-surf'
    ]"
);

mod client;
mod error;
mod ids;
mod params;
mod resources;

// N.B. Ideally we would support both a blocking client and
//      an async client without a feature flag, but the originally
//      discussed solution requires Generic Associated Types--
//      instead we provide an async client only a feature flag.
//
// See https://github.com/wyyerd/stripe-rs/issues/24#issuecomment-451514187
// See https://github.com/rust-lang/rust/issues/44265
pub use crate::client::*;
pub use crate::error::{ErrorCode, ErrorType, RequestError, StripeError, WebhookError};
pub use crate::ids::*;
pub use crate::params::{
    Expandable, Headers, IdOrCreate, List, Metadata, Object, RangeBounds, RangeQuery, Timestamp,
};
pub use crate::resources::*;
