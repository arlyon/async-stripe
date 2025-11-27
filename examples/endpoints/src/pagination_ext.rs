#![allow(unused_variables, dead_code)]

//! PaginationExt
//! =============
//!
//! This example demonstrates using the PaginationExt trait to paginate
//! through nested list objects like checkout session line items.

use std::str::FromStr;

use futures_util::TryStreamExt;
use stripe::{Client, PaginationExt, StripeError};
use stripe_checkout::{CheckoutSessionId, checkout_session::RetrieveCheckoutSession};

pub async fn print_all_line_items(client: &Client, session_id: &str) -> Result<(), StripeError> {
    // Retrieve the session, expanding line_items
    let session =
        RetrieveCheckoutSession::new(CheckoutSessionId::from_str(session_id).expect("infallible"))
            .expand(["line_items".to_string()])
            .send(client)
            .await?;

    if let Some(line_items_list) = session.line_items {
        // line_items_list is a List<LineItem>.
        // To fetch subsequent pages, convert it into a paginator:
        let mut stream = line_items_list
            .into_paginator() // Available because of PaginationExt
            .stream(client);

        while let Some(item) = stream.try_next().await? {
            println!("Line item: {:?}", item.description);
        }
    }

    Ok(())
}
