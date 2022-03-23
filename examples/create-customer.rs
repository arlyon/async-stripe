use common::create_customer;
use stripe::{Client, Customer};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let customer = create_customer(&client).await;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);
}
