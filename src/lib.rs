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
//! To get started, we need to create a client:
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
//! ```
//! /* Creating a Stripe Charge */
//! # #[cfg(feature = "blocking")]
//! # {
//! # let client = stripe::Client::from_url("http://localhost:12111", "sk_test_123");
//! let token = "tok_ID_FROM_CHECKOUT".parse().unwrap();
//! let mut params = stripe::CreateCharge::new();
//! // NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
//! params.amount = Some(1095); // e.g. $10.95
//! params.source = Some(stripe::ChargeSourceParams::Token(token));
//!
//! // Example: Override currency to be in Canadian Dollars
//! params.currency = Some(stripe::Currency::CAD);
//! let charge = stripe::Charge::create(&client, params).unwrap();
//! println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }
//! # }
//! ```
//!
//! ```
//! /* Listing Stripe Charges */
//! # #[cfg(feature = "blocking")]
//! # {
//! # let client = stripe::Client::from_url("http://localhost:12111", "sk_test_123");
//! let params = stripe::ListCharges::new();
//! let charges = stripe::Charge::list(&client, params).unwrap();
//! println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
//! # }
//! ```
//!
//! > **A note about creating card tokens**: Stripe introduced the [PaymentIntent](crate::PaymentIntent) api
//! > to replace the old token and charge API. This library only supports the former. To migrate, you can
//! > have a look at [the official migration guide](https://stripe.com/docs/payments/payment-intents/migration).

#![allow(clippy::map_clone)]
#![allow(clippy::large_enum_variant)]
#![warn(clippy::unwrap_used)]
#![forbid(unsafe_code)]

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
