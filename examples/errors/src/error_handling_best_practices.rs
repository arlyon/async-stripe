//! Error Handling Best Practices
//! ==============================
//!
//! This example demonstrates best practices for error handling including:
//! - Handling specific errors differently
//! - Using idempotency keys for critical operations
//! - Logging error details for debugging

use stripe::{Client, IdempotencyKey, RequestStrategy, StripeError, StripeRequest};
use stripe_core::payment_intent::{CreatePaymentIntent, RetrievePaymentIntent};
use stripe_types::Currency;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    // Best Practice 1: Handle specific errors differently
    handle_specific_errors(&client).await;

    // Best Practice 2: Use idempotency keys for critical operations
    use_idempotency_key(&client).await;

    // Best Practice 3: Log error details
    log_error_details(&client).await;
}

// Best Practice 1: Handle Specific Errors
async fn handle_specific_errors(client: &Client) {
    let result = RetrievePaymentIntent::new("pi_example").send(client).await;

    match result {
        Err(StripeError::Stripe(api_error, 402)) => {
            // Show user-friendly message for payment failures
            show_payment_error_to_user(&api_error);
        }
        Err(StripeError::Stripe(api_error, 400)) => {
            // Log parameter errors for debugging
            error!("Invalid parameters: {:?}", api_error);
        }
        Err(e) => {
            // Log unexpected errors and alert monitoring
            error!("Unexpected Stripe error: {}", e);
        }
        Ok(_result) => {
            // Success
            info!("Operation succeeded");
        }
    }
}

fn show_payment_error_to_user(api_error: &stripe::ApiErrors) {
    if let Some(message) = &api_error.message {
        info!("Payment failed: {}", message);
    }
}

// Best Practice 2: Use Idempotency Keys
async fn use_idempotency_key(client: &Client) {
    let key = IdempotencyKey::new("order_12345").unwrap();

    let payment = CreatePaymentIntent::new(1000, Currency::USD)
        .customize()
        .request_strategy(RequestStrategy::Idempotent(key))
        .send(client)
        .await;

    match payment {
        Ok(payment_intent) => {
            info!("Created payment intent: {}", payment_intent.id);
        }
        Err(e) => {
            error!("Failed to create payment: {}", e);
        }
    }
}

// Best Practice 3: Log Error Details
async fn log_error_details(client: &Client) {
    let result = RetrievePaymentIntent::new("pi_example").send(client).await;

    if let Err(StripeError::Stripe(api_error, status)) = result {
        error!(
            status = status,
            error_type = ?api_error.type_,
            code = ?api_error.code,
            message = ?api_error.message,
            param = ?api_error.param,
            "Stripe error"
        );
    }
}
