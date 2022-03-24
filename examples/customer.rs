//! Customer
//! ========
//!
//! Reference: <https://stripe.com/docs/api/customers>
//!
//! This example shows how to create and list customers.

use stripe::{Client, CreateCustomer, Customer};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let customer = Customer::create(
        &client,
        CreateCustomer {
            name: Some("Alexander Lyon"),
            email: Some("test@async-stripe.com"),
            description: Some(
                "A fake customer that is used to illustrate the examples in async-stripe.",
            ),
            metadata: Some(
                [("async-stripe".to_string(), "true".to_string())].iter().cloned().collect(),
            ),

            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    let all_customers = Customer::list(&client, Default::default()).await.unwrap();

    println!(
        "customers: {:#?}",
        all_customers.data.iter().map(|c| c.name.as_ref().unwrap()).collect::<Vec<_>>()
    );
}
