#[cfg(feature = "async")]
mod checkout;
#[cfg(feature = "async")]
mod connect;
#[cfg(feature = "async")]
mod customer;
#[cfg(feature = "async")]
mod payment_intent;
#[cfg(feature = "async")]
mod payment_link;
#[cfg(feature = "async")]
mod subscriptions;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), stripe::StripeError> {
    use stripe::Client;

    use crate::checkout::run_checkout_session_example;
    use crate::connect::run_connect_example;
    use crate::customer::run_customer_example;
    use crate::payment_intent::run_payment_intent_example;
    use crate::payment_link::run_payment_link_example;
    use crate::subscriptions::run_subscriptions_example;

    let secret_key =
        std::env::var("STRIPE_TEST_SECRET_KEY").expect("Missing STRIPE_TEST_SECRET_KEY in env");
    let client = Client::new(secret_key);
    run_checkout_session_example(&client).await?;
    run_connect_example(&client).await?;
    run_customer_example(&client).await?;
    run_payment_intent_example(&client).await?;
    run_payment_link_example(&client).await?;
    run_subscriptions_example(&client).await?;
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {}
