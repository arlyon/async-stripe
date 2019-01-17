use futures::future::Future;

fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::r#async::Client::new(secret_key);

    // Define a card to charge
    let mut card = stripe::CardParams::default();
    card.number = "4242424242424242";
    card.exp_month = "10";
    card.exp_year = "20";

    // Define the charge
    let mut params = stripe::ChargeParams::default();
    params.amount = Some(1000);
    params.source = Some(stripe::PaymentSourceParams::Card(card));

    // Create the charge
    stripe::Charge::create(&client, params)
        .map(|charge| {
            println!("{:?}", charge);
        })
        .wait()
        .unwrap();
}
