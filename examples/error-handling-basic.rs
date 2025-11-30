//! Basic Error Handling
//! =====================
//!
//! This example demonstrates basic error handling patterns when working with the Stripe API.
//! It shows how to match on different error types and handle them appropriately.

use stripe::{Client, CreateCustomer, Customer, StripeError};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    match Customer::create(&client, CreateCustomer::new()).send().await {
        Ok(customer) => info!("Created customer: {}", customer.id),
        Err(err) => match err {
            StripeError::Stripe(api_error, status_code) => {
                error!("Stripe API error ({}): {:?}", status_code, api_error.message);
            }
            StripeError::ClientError(msg) => {
                error!("Network error: {}", msg);
            }
            _ => {
                error!("Other error: {}", err);
            }
        },
    }
}
