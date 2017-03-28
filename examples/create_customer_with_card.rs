extern crate stripe;

use std::env;

fn main() {
    // Create a new client
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(&secret_key);

    // Build request params
    let mut params = stripe::CustomerParams::default();
    params.email = Some("jdoe@example.org");
    params.source = Some(stripe::CustomerSource::Card(
        stripe::CardParams{
            object: "card",
            number: "4242424242424242",
            exp_month: "02",
            exp_year: "21",

            name: None,
            cvc: None,
        }
    ));

    // Perform request
    let customer = stripe::Customer::create(&client, params).unwrap();

    // Output in a ~prettyprint format
    println!("Customer {{
    id: {:?},
    created: {:?},
    default_source: {:?},
    email: {:?},
    ..
}}", customer.id, customer.created, customer.default_source, customer.email);
}
