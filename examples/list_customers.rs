extern crate stripe;

use std::env;
use stripe::{Customer, CustomerListParams, RangeQuery, RangeOp};

fn main() {
    // Create a new client
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // List customer
    let customers = Customer::list(
        &client,
        CustomerListParams {
            created: Some(RangeQuery::Op(RangeOp::Gte(1501598702))),
            ending_before: None,
            limit: Some(3),
            starting_after: None,
        },
    ).unwrap();

    // Print the first three customers
    println!("{:?}", customers);

    // List the next three customers (using default)
    let mut params = CustomerListParams::default();
    params.limit = Some(3);
    params.starting_after = customers.data.last().map(|cust| cust.id.as_str());
    let customers = Customer::list(&client, params).unwrap();

    // Print the following three customers
    println!("{:?}", customers);
}
