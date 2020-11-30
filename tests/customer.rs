mod mock;

#[cfg(feature = "blocking")]
fn customer_create_and_delete(client: &stripe::Client) {
    let customer_params = stripe::CreateCustomer::new();
    let customer = stripe::Customer::create(client, customer_params).unwrap();
    let result = stripe::Customer::delete(client, &customer.id);
    match result {
        Ok(deleted) => assert!(deleted.deleted, "customer wasn't deleted"),
        Err(err) => assert!(false, "{}", err),
    }
}

#[test]
#[cfg(feature = "blocking")]
fn customer_create_and_delete_without_account() {
    mock::with_client(|client| {
        customer_create_and_delete(client);
    });
}

#[test]
#[cfg(feature = "blocking")]
fn customer_create_and_delete_with_account() {
    mock::with_client(|client| {
        let client = client.with_headers(stripe::Headers {
            stripe_account: Some("TEST".into()),
            client_id: Some("ca_123".into()),
            stripe_version: Some(stripe::ApiVersion::V2019_03_14),
            user_agent: None,
        });
        customer_create_and_delete(&client);
    });
}
