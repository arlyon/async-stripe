#[tokio::main]
async fn main() -> Result<(), stripe::StripeError> {
    use futures_util::TryStreamExt;
    use stripe::Client;
    use stripe_core::customer::ListCustomer;

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);
    let paginator = ListCustomer::new().paginate();
    let mut stream = paginator.stream(&client);

    // take a value out from the stream
    if let Some(val) = stream.try_next().await? {
        println!("GOT = {:?}", val);
    }

    // alternatively, you can use stream combinators
    let _ = stream.try_collect::<Vec<_>>().await?;
    Ok(())
}
