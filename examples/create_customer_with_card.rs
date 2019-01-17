fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // Define a card for the customer
    let mut card = stripe::CardParams::default();
    card.number = "4242424242424242";
    card.exp_month = "10";
    card.exp_year = "20";

    // Define the customer
    let mut params = stripe::CustomerParams::default();
    params.email = Some("jdoe@example.org");
    params.source = Some(stripe::PaymentSourceParams::Card(card));

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
