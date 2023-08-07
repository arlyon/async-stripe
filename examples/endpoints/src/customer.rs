//! Customer
//! ========
//!
//! Reference: <https://stripe.com/docs/api/customers>
//!
//! This example shows how to create and list customers.

use futures_util::StreamExt;
use futures_util::TryStreamExt;
use stripe::{Client, StripeError};
use stripe_core::customer::{CreateCustomer, ListCustomer};

pub async fn run_customer_example(client: &Client) -> Result<(), StripeError> {
    let meta =
        std::collections::HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    let customer = CreateCustomer {
        name: Some("Alexander Lyon"),
        email: Some("test@async-stripe.com"),
        description: Some(
            "A fake customer that is used to illustrate the examples in async-stripe.",
        ),
        metadata: Some(&meta),
        ..Default::default()
    }
    .send(client)
    .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    let customer = CreateCustomer {
        name: Some("Someone Else"),
        email: Some("test@async-stripe.com"),
        description: Some(
            "A fake customer that is used to illustrate the examples in async-stripe.",
        ),
        metadata: Some(&meta),
        ..Default::default()
    }
    .send(client)
    .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    let mut stream = ListCustomer::new().paginate().stream(client);

    // get the next customer
    let _next = stream.next().await.unwrap();

    // or collect them
    let customers = stream.try_collect::<Vec<_>>().await.unwrap();

    println!("fetched {} customers: {:?}", customers.len(), customers);
    Ok(())
}
