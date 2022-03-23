use stripe::{Client, CreateCustomer, Customer};

pub async fn create_customer(client: &Client) -> Customer {
    Customer::create(
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
    .unwrap()
}
