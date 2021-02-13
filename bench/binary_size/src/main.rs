// N.B. While it should be worthwhile to include other APIs in the binary size benchmark,
//      it isn't super important because a lot of the binary weight (e.g. from `serde`, etc)
//      is still included even after dead code elimination due to the recursive dependency
//      on types caused by `Expand`.
// TODO: Use other apis

#[cfg(feature = "runtime-blocking")]
fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);
    create_charge(&client);
    create_customer(&client);
}

#[cfg(feature = "runtime-blocking")]
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

#[cfg(feature = "runtime-blocking")]
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

#[cfg(feature = "runtime-async")]
#[async_std::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);
    create_charge(&client).await;
    create_customer(&client).await;
}

#[cfg(feature = "runtime-async")]
async fn create_charge(client: &stripe::Client) {
    // Define a card to charge
    let card = "card_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected card to be valid");

    // Define the charge
    let mut params = stripe::CreateCharge::new();
    params.amount = Some(1000);
    params.source = Some(stripe::ChargeSourceParams::Card(card));

    // Create the charge
    let charge = stripe::Charge::create(&client, params).await.unwrap();

    // Output the result
    println!("{:?}", charge);
}

#[cfg(feature = "runtime-async")]
async fn create_customer(client: &stripe::Client) {
    // Define the customer
    let token = "tok_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected token to be valid");
    let mut params = stripe::CreateCustomer::new();
    params.source = Some(stripe::PaymentSourceParams::Token(token));
    params.email = Some("jdoe@example.org");

    // Create the customer
    let customer = stripe::Customer::create(&client, params).await.unwrap();

    // Output the result
    println!("{:?}", customer);
}
