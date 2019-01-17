// Copyright 2019 Wyyerd Group, LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/stripe-rust/")]

//! This crate provides Rust bindings to the Stripe HTTP API.
//!
//! ## Getting Started
//!
//! To get started, we need to create a client:
//!
//! ```rust
//!   let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! ```
//!
//! Then we can begin making requests as we'd like.  Most Stripe requests accept
//! many optional parameters, so we usually get the `::default()` params and then
//! set the ones we want from there.
//!
//! Most requests for creating or updating a Stripe object use the same Rust struct,
//! so you may frequently need to refer to the [official API docs](https://stripe.com/docs/api)
//! to determine which fields are required for either request.
//!
//! ```rust,no_run
//! /* Creating a Stripe Charge */
//!
//! # let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! let token = "tok_ID_FROM_CHECKOUT".parse().unwrap();
//! let mut params = stripe::ChargeParams::default();
//! // NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
//! params.amount = Some(1095); // e.g. $10.95
//! params.source = Some(stripe::PaymentSourceParams::Token(token));
//!
//! // Example: Override currency to be in Canadian Dollars
//! params.currency = Some(stripe::Currency::CAD);
//! let charge = stripe::Charge::create(&client, params).unwrap();
//! println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }
//! ```
//!
//! ```rust,no_run
//! /* Listing Stripe Charges */
//!
//! # let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
//! let params = stripe::ChargeListParams::default();
//! let charges = stripe::Charge::list(&client, params).unwrap();
//! println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
//! ```

#![deny(warnings)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::large_enum_variant)]

#[cfg(not(feature = "async"))]
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
#[cfg(feature = "async")]
pub mod r#async;
#[cfg(not(feature = "async"))]
pub use crate::client::Client;
pub use crate::error::{Error, ErrorCode, ErrorType, RequestError, WebhookError};
pub use crate::ids::*;
pub use crate::params::{Headers, List, Metadata, RangeBounds, RangeQuery, Timestamp};
pub use crate::resources::*;

#[cfg(not(feature = "async"))]
mod config {
    use crate::error::Error;

    /// An alias for `Result`.
    ///
    /// ```rust,ignore
    /// type Response<T> = Result<T, Error>;
    /// ```
    ///
    /// If the `async` feature is enabled, this type is redefined as:
    ///
    /// ```rust,ignore
    /// type Response<T> = Box<dyn Future<Item = T, Error = Error> + Send>
    /// ```
    pub type Response<T> = Result<T, Error>;

    pub(crate) type Client = crate::client::Client;

    #[inline]
    pub fn ok<T>(ok: T) -> Response<T> {
        Ok(ok)
    }

    #[inline]
    pub fn err<T>(err: Error) -> Response<T> {
        Err(err)
    }
}

#[cfg(feature = "async")]
mod config {
    use crate::error::Error;
    use futures::future::{self, Future};

    // TODO: We'd rather use `impl Future<Result<T, Error>>` but that isn't so
    //       easy to accomplish in generic code with futures 0.1.x
    pub type Response<T> = Box<dyn Future<Item = T, Error = Error> + Send>;

    pub(crate) type Client = crate::r#async::Client;

    #[inline]
    pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
        Box::new(future::ok(ok))
    }

    #[inline]
    pub(crate) fn err<T: Send + 'static>(err: Error) -> Response<T> {
        Box::new(future::err(err))
    }
}

// N.B. export for doc purposes
pub use self::config::Response;
