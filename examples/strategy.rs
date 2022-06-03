//! Payment Link
//! ============
//!
//! Reference: <https://stripe.com/docs/api/payment_links/payment_links>
//!
//! This example shows how to create a payment link for
//! a particular product and price. The nice thing with
//! this API is the lack of associated customer.

use stripe::{Client, Customer, ListCustomers, RequestStrategy};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key).with_strategy(RequestStrategy::idempotent_with_uuid());

    let first_page =
        Customer::list(&client, &ListCustomers { limit: Some(1), ..Default::default() })
            .await
            .unwrap();

    println!(
        "first page of customers: {:#?}",
        first_page.data.iter().map(|c| c.name.as_ref().unwrap()).collect::<Vec<_>>()
    );
}
