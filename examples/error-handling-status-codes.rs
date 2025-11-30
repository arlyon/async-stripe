//! HTTP Status Code Error Handling
//! ================================
//!
//! This example demonstrates how to handle different HTTP status codes returned by the Stripe API.
//! Different status codes indicate different types of failures and should be handled differently.

use stripe::{Client, PaymentIntent, StripeError};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    // Replace with an actual payment intent ID for testing
    let payment_intent_id = "pi_example";

    match PaymentIntent::retrieve(&client, &payment_intent_id, &[]).await {
        Ok(payment) => {
            info!("Payment status: {:?}", payment.status);
        }
        Err(StripeError::Stripe(api_error, status)) => match status {
            400 => {
                // Bad Request - Invalid parameters
                error!("Invalid request: {:?}", api_error.message);
            }
            401 => {
                // Unauthorized - Invalid API key
                error!("Authentication failed - check your API key");
            }
            402 => {
                // Payment Required - Card declined or insufficient funds
                error!("Payment failed: {:?}", api_error.message);

                // The error may contain a decline code
                if let Some(code) = &api_error.code {
                    match code.as_str() {
                        "card_declined" => error!("Card was declined"),
                        "insufficient_funds" => error!("Insufficient funds"),
                        "expired_card" => error!("Card has expired"),
                        _ => error!("Payment error code: {}", code),
                    }
                }
            }
            404 => {
                // Not Found - Resource doesn't exist
                error!("Resource not found");
            }
            429 => {
                // Too Many Requests - Rate limited
                warn!("Rate limited - slow down requests");
            }
            500 | 502 | 503 | 504 => {
                // Server errors - retry with backoff
                error!("Stripe server error - retry later");
            }
            _ => {
                error!("HTTP {}: {:?}", status, api_error.message);
            }
        },
        Err(e) => {
            error!("Non-API error: {}", e);
        }
    }
}
