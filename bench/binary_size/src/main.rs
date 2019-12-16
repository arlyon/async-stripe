fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // Use the charge api
    create_charge(&client);

    // Use the customer api
    create_customer(&client);

    // N.B. While it should be worthwhile to include other APIs in the binary size benchmark,
    //      it isn't super important because a lot of the binary weight (e.g. from `serde`, etc)
    //      is still included even after dead code elimination due to the recursive dependency
    //      on types causes by `Expand`.
    // TODO: Use other apis
}

fn create_charge(client: &stripe::Client) {
    // Define a card to charge
    let card = "card_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected card to be valid");

    // Define the charge
    let mut params = stripe::CreateCharge::new();
    params.amount = Some(1000);
    params.source = Some(stripe::ChargeSourceParams::Card(card));

    // Create the charge
    let charge = stripe::Charge::create(&client, params).unwrap();

    // Output the result
    println!("{:?}", charge);
}

fn create_customer(client: &stripe::Client) {
    // Define the customer
    let token = "tok_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected token to be valid");
    let mut params = stripe::CreateCustomer::new();
    params.source = Some(stripe::PaymentSourceParams::Token(token));
    params.email = Some("jdoe@example.org");

    // Create the customer
    let customer = stripe::Customer::create(&client, params).unwrap();

    // Output the result
    println!("{:?}", customer);
}
