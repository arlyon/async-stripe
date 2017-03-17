extern crate stripe;

use std::env;

fn main() {
    // Create a new client
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // TODO: Keep track of https://github.com/rust-lang/rust-roadmap/issues/17
    //       so we can use default struct field value syntax eventually
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

/* Creating a customer without using default:

    let customer = Customer::create(&client, CustomerParams{
        email: Some("jdoe@example.org"),
        source: Some(CustomerSource::Token("token_from_checkout")),

        account_balance: None,
        business_vat_id: None,
        coupon: None,
        description: None,
        metadata: None,
        shipping: None,
    }).unwrap();
*/
