use stripe_core::customer::{
    CreateCustomer, DeleteCustomer, RetrieveCustomer, RetrieveCustomerReturned,
};

use crate::mock::get_client;

fn customer_create_and_delete(client: &stripe::Client) {
    // NB: the create step is not required for deletion to work since the stripe mock is stateless
    let customer = CreateCustomer::new().send(client).unwrap();
    let result = DeleteCustomer::new().send(client, &customer.id).unwrap();
    assert_eq!(result.id, customer.id);
}

#[test]
fn customer_create_and_delete_without_account() {
    let client = get_client();
    customer_create_and_delete(&client);
}

#[test]
fn customer_create_and_delete_with_account() {
    let client = get_client()
        .with_client_id("ca_123".parse().unwrap())
        .with_stripe_account("acct_123".parse().unwrap());
    customer_create_and_delete(&client);
}

#[test]
fn retrieve_customer() {
    let client = get_client();
    let id = "cus_123".parse().unwrap();
    let ret = RetrieveCustomer::new().send(&client, &id).unwrap();
    match ret {
        RetrieveCustomerReturned::Customer(cust) => {
            assert_eq!(cust.id, id);
            assert_eq!(cust.invoice_prefix, Some("B4749BD".into()));
            assert_eq!(cust.created, 1234567890);
        }
        RetrieveCustomerReturned::DeletedCustomer(_) => panic!("expected non-deleted response"),
    }
}
