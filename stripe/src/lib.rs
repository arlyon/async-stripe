// Copyright 2017 Rapidity Networks, Inc.
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
//! ```rust,ignore
//! /* Creating a Stripe Charge */
//!
//! let token = "TOKEN_FROM_CHECKOUT";
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
//! ```rust,ignore
//! /* Listing Stripe Charges */
//!
//! let params = stripe::ChargeListParams::default();
//! let charges = stripe::Charge::list(&client, params).unwrap();
//! println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
//! ```

#![deny(warnings)]

extern crate chrono;
extern crate hmac;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;
extern crate sha2;

mod client;
mod error;
mod ids;
mod params;
mod resources;

pub use client::{Client, Params};
pub use error::{Error, ErrorCode, ErrorType, RequestError, WebhookError};
pub use ids::*;
pub use params::{List, Metadata, RangeBounds, RangeQuery, Timestamp};
pub use resources::*;
