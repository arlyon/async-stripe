mod mock;

#[cfg(feature = "blocking")]
fn customer_create_and_delete(client: &stripe::Client) {
    let customer_params = stripe::CreateCustomer::new();
    let customer = stripe::Customer::create(client, customer_params).unwrap();
    let result = stripe::Customer::delete(client, &customer.id);
    match result {
        Ok(deleted) => assert!(deleted.deleted, "customer wasn't deleted"),
        Err(err) => panic!("{}", err),
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
        let client = client
            .to_owned()
            .with_client_id("ca_123".parse().unwrap())
            .with_stripe_account("acct_123".parse().unwrap());
        customer_create_and_delete(&client);
    });
}
