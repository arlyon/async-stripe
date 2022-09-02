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

pub mod resources;
