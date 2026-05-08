#![allow(unused_variables, dead_code)]

//! Request Timeouts
//! ================
//!
//! This example shows how to configure per-attempt timeouts on the client,
//! override them per request, and enforce a deadline for the entire logical
//! request including retries.

use std::time::Duration;

use stripe::{Client, ClientBuilder, StripeError, StripeRequest};
use stripe_core::customer::ListCustomer;

pub async fn run_timeout_example() -> Result<(), StripeError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");

    let client = ClientBuilder::new(secret_key)
        .timeout(Duration::from_secs(10))
        .build()?;

    let customers = ListCustomer::new()
        .customize()
        .timeout(Duration::from_secs(2))
        .send(&client)
        .await?;

    println!("fetched {} customers", customers.data.len());
    Ok(())
}

pub async fn total_request_timeout_example(client: &Client) -> Result<(), StripeError> {
    let customers = tokio::time::timeout(
        Duration::from_secs(10),
        ListCustomer::new().customize().send(client),
    )
    .await
    .map_err(|_| StripeError::Timeout)??;

    println!("fetched {} customers before the overall deadline", customers.data.len());
    Ok(())
}
