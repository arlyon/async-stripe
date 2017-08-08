extern crate stripe;

use std::env;
use stripe::{Customer, CustomerListParams, RangeQuery, RangeOp};

fn main() {
    // Create a new client
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // Create the customer
    let customers = Customer::list(&client,
                                   CustomerListParams {
                                       created: Some(RangeQuery::Op(RangeOp::Gte(1501598702))),
                                       ending_before: None,
                                       limit: Some(3),
                                       starting_after: None,
                                   })
        .unwrap();


    // Output in a ~prettyprint format
    println!("{:?}", customers);
}
