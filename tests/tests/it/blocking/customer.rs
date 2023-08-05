use stripe_core::customer::{CreateCustomer, DeleteCustomer};

use crate::mock;

fn customer_create_and_delete(client: &stripe::Client) {
    // NB: the create step is not required for deletion to work since the stripe mock is stateless
    let customer = CreateCustomer::new().send(client).unwrap();
    let result = DeleteCustomer::new().send(client, &customer.id).unwrap();
    assert_eq!(result.id, customer.id);
}

#[test]
fn customer_create_and_delete_without_account() {
    mock::with_client(|client| {
        customer_create_and_delete(client);
    });
}

#[test]
fn customer_create_and_delete_with_account() {
    mock::with_client(|client| {
        let client = client.to_owned().with_client_id("ca_123".parse().unwrap()).with_stripe_account("acct_123".parse().unwrap());
        customer_create_and_delete(&client);
    });
}
