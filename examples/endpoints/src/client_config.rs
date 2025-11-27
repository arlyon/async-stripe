#![allow(unused_variables, dead_code)]

//! Client Configuration
//! ====================
//!
//! This example demonstrates advanced client configuration including
//! app info and Stripe Connect account masquerading.

use stripe::{ClientBuilder, StripeError};

pub async fn advanced_client_config() -> Result<(), StripeError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");

    let client = ClientBuilder::new(secret_key)
        // Identify your app to Stripe (useful for analytics and support)
        .app_info("MyApp", Some("1.0.0".to_string()), Some("https://myapp.com".to_string()))
        // Acts as this connected account (Stripe Connect)
        .account_id(stripe::AccountId::from("acct_12345"))
        .build()?;

    println!("Client configured with app info and account masquerading");

    Ok(())
}
