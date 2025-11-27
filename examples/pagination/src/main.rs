use futures_util::TryStreamExt;
use stripe::Client;
use stripe_core::customer::ListCustomer;

#[tokio::main]
async fn main() -> Result<(), stripe::StripeError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);
    let paginator = ListCustomer::new().paginate();
    let mut stream = paginator.stream(&client);

    // take a value out from the stream
    if let Some(val) = stream.try_next().await? {
        println!("GOT = {val:?}");
    }

    // alternatively, you can use stream combinators
    let _ = stream.try_collect::<Vec<_>>().await?;
    Ok(())
}

pub async fn manual_pagination_example(client: &Client) -> Result<(), stripe::StripeError> {
    let mut params = ListCustomer::new().limit(10);

    loop {
        let page = params.send(client).await?;
        if page.data.is_empty() {
            break;
        }

        for customer in &page.data {
            println!("{}", customer.id);
        }

        if !page.has_more {
            break;
        }

        // Set cursor for next loop
        if let Some(last) = page.data.last() {
            params = params.starting_after(last.id.as_str());
        }
    }

    Ok(())
}
