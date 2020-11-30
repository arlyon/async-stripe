fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // Create the customer
    let token = "tok_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected token to be valid");
    let mut params = stripe::CreateCustomer::new();
    params.source = Some(stripe::PaymentSourceParams::Token(token));
    params.email = Some("jdoe@example.org");
    let customer = stripe::Customer::create(&client, params).unwrap();

    // Output in a ~prettyprint format
    println!(
        "Customer {{
    id: {:?},
    created: {:?},
    default_source: {:?},
    email: {:?},
    ..
}}",
        customer.id, customer.created, customer.default_source, customer.email
    );
}
