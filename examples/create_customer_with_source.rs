fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    let source = "src_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected source to be valid");

    // Define the customer
    let mut params = stripe::CreateCustomer::new();
    params.email = Some("jdoe@example.org");
    params.source = Some(stripe::PaymentSourceParams::Source(source));

    // Perform request
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
