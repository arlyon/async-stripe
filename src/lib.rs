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
//! > the documentation. Any time an API is used in an example it is highlighted in the
//! > docs for that item. You can also find all the raw examples in the `examples` directory.
//! > Please have a look at those for inspiration or ideas on how to get started.
//!
//! ## Idempotency / Request Strategies
//!
//! This library provides a few basic request strategies for making requests to the Stripe API.
//! This is currently implemented as an enum with the following variants:
//!
//! - [`RequestStrategy::Once`]: This is the default strategy. It will make a request to the Stripe API and,
//!                              whether the request fails or not, will simply return the response.
//! - [`RequestStrategy::Idempotent`]: This strategy will make a request to stripe api, passing the provided
//!                                    key to Stripe as the `Idempotency-Key` header, ensuring that the request
//!                                    is idempotent. If the request fails, you may retry it.
//! - [`RequestStrategy::Retry`]: Make a request to the Stripe API and, if the request fails, retry it up to n
//!                               times with a timeout. The idempotency key is generated  automatically and is
//!                               stable across retries.
//! - [`RequestStrategy::ExponentialBackoff`]: Make a request to the Stripe API and, if the request fails, retry
//!                                            it up to n times with exponential backoff. The idempotency key is
//!                                            generated automatically and is stable across retries.
//!
//! > Want to implement your own? If it is a common strategy, please consider opening a PR to add it to the library.
//! > Otherwise, we are open to turning this into an open trait so that you can implement your own strategy.

#![allow(clippy::large_enum_variant)]
#![warn(clippy::missing_panics_doc)]
#![forbid(unsafe_code)]
// Workaround
#![allow(ambiguous_glob_reexports)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Give a clear error when a required runtime error is not present. Would be better for this
// to be a fatal error preventing emission of further compile errors relating to lack of
// a runtime feature, but that does not seem currently possible:
// https://github.com/rust-lang/rust/issues/68838

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
    Expandable, Headers, IdOrCreate, List, Metadata, Object, RangeBounds, RangeQuery, SearchList,
    Timestamp,
};
pub use crate::resources::*;
