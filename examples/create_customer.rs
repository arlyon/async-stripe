extern crate stripe;

use std::env;
use stripe::{CardParams, Customer, CustomerParams, CustomerSource};

fn main() {
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let customer = Customer::create(CustomerParams{
        email: Some("jdoe@example.org".to_string()),
        source: Some(CustomerSource::Card(CardParams{
            object: "card",
            number: "4242424242424242".to_string(),
            exp_month: "02".to_string(),
            exp_year: "21".to_string(),

            name: None,
            cvc: None,
        })),

        account_balance: None,
        business_vat_id: None,
        coupon: None,
        description: None,
        metadata: None,
        shipping: None,
    }, &secret_key).unwrap();

    // Output in a ~prettyprint format
    println!("Customer {{
    id: {:?},
    created: {:?},
    default_source: {:?},
    email: {:?},
    ..
}}", customer.id, customer.created, customer.default_source, customer.email);
}
