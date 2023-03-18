//! Customer
//! ========
//!
//! Reference: <https://stripe.com/docs/api/customers>
//!
//! This example shows how to create and list customers.

use futures_util::StreamExt;
use futures_util::TryStreamExt;
use stripe::{Client, CreateCustomer, Customer, ListCustomers};

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
            metadata: Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )])),

            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    let customer = Customer::create(
        &client,
        CreateCustomer {
            name: Some("Someone Else"),
            email: Some("test@async-stripe.com"),
            description: Some(
                "A fake customer that is used to illustrate the examples in async-stripe.",
            ),
            metadata: Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )])),

            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    let params = ListCustomers { ..Default::default() };
    let paginator = Customer::list(&client, &params).await.unwrap().paginate(params);
    let mut stream = paginator.stream(&client);

    // get the next customer
    let _next = stream.next().await.unwrap();

    // or collect them
    let customers = stream.try_collect::<Vec<_>>().await.unwrap();

    println!("fetched {} customers: {:?}", customers.len(), customers);
}
