use crate::checkout::run_checkout_session_example;
use crate::connect::run_connect_example;

#[cfg(feature = "async")]
mod checkout;
#[cfg(feature = "async")]
mod connect;
// mod customer;
// mod payment_intent;
// mod payment_link;
// mod subscriptions;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), stripe::StripeError> {
    use stripe::Client;

    let secret_key =
        std::env::var("STRIPE_TEST_SECRET_KEY").expect("Missing STRIPE_TEST_SECRET_KEY in env");
    let client = Client::new(secret_key);
    run_checkout_session_example(&client).await?;
    run_connect_example(&client).await?;
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {}
