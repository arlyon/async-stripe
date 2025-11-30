//! Retry Strategies
//! ================
//!
//! This example demonstrates how to use the built-in retry strategies for handling
//! transient errors like network issues and server errors.

use stripe::{Client, Customer, RequestStrategy};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");

    let client = Client::builder(secret_key)
        .request_strategy(RequestStrategy::ExponentialBackoff(3))
        .build();

    // Replace with an actual customer ID for testing
    let customer_id = "cus_example";

    // This request will automatically retry up to 3 times with backoff
    let customer = Customer::retrieve(&client, &customer_id, &[]).await;

    match customer {
        Ok(customer) => info!("Retrieved customer: {}", customer.id),
        Err(e) => error!("Failed to retrieve customer after retries: {}", e),
    }
}
