use futures::future::Future;

fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::r#async::Client::new(secret_key);

    // Define a card to charge
    let mut card = "card_189g322eZvKYlo2CeoPw2sdy".parse().expect("expected card to be valid");

    // Define the charge
    let mut params = stripe::CreateCharge::new();
    params.amount = Some(1000);
    params.source = Some(stripe::PaymentChargeParams::Card(card));

    // Create the charge
    stripe::Charge::create(&client, params)
        .map(|charge| {
            println!("{:?}", charge);
        })
        .wait()
        .unwrap();
}
