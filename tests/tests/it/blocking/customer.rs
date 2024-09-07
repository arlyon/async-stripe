use stripe_core::{
    customer::{CreateCustomer, DeleteCustomer, RetrieveCustomer, RetrieveCustomerReturned},
    CustomerId,
};

use super::{get_base_test_config, get_client};

fn customer_create_and_delete(client: &stripe::blocking::Client) {
    // NB: the create step is not required for deletion to work since the stripe mock is stateless
    let customer = CreateCustomer::new().send_blocking(client).unwrap();
    let result = DeleteCustomer::new(&customer.id).send_blocking(client).unwrap();
    assert_eq!(result.id, customer.id);
}

#[test]
fn customer_create_and_delete_without_account() {
    let client = get_client();
    customer_create_and_delete(&client);
}

#[test]
fn customer_create_and_delete_with_account() {
    let client = get_base_test_config()
        .client_id("ca_123".parse().unwrap())
        .account_id("acct_123".parse().unwrap())
        .build_sync()
        .unwrap();
    customer_create_and_delete(&client);
}

#[test]
fn retrieve_customer() {
    let client = get_client();
    let id = CustomerId::from("cus_123");
    let ret = RetrieveCustomer::new(&id).send_blocking(&client).unwrap();
    match ret {
        RetrieveCustomerReturned::Customer(cust) => {
            assert_eq!(cust.id, id);
            assert_eq!(cust.invoice_prefix, Some("B4749BD".into()));
            assert_eq!(cust.created, 1234567890);
        }
        RetrieveCustomerReturned::DeletedCustomer(_) => panic!("expected non-deleted response"),
    }
}
