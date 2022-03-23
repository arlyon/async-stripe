//! Stripe connect
//! ==============
//!
//! This example runs through how to manange a Stripe connect account.
//! To get started, you'll need to make sure you have signed up to use
//! stripe connect and configure branding settings with an icon and a
//! brand color. See more: https://dashboard.stripe.com/connect/accounts/overview
//!
//! The example below initiates an account link which can be used to
//! onboard a new user to your application.

use stripe::{
    Client, CreatePaymentLink, CreatePaymentLinkLineItems, CreatePrice, CreateProduct, Currency,
    IdOrCreate, PaymentLink, Price, Product,
};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    // create a new exmaple project
    let product = {
        let mut create_product = CreateProduct::new("T-Shirt");
        create_product.metadata =
            Some([("async-stripe".to_string(), "true".to_string())].iter().cloned().collect());
        Product::create(&client, create_product).await.unwrap()
    };

    // and add a price for it in USD
    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.metadata =
            Some([("async-stripe".to_string(), "true".to_string())].iter().cloned().collect());
        create_price.unit_amount = Some(1000);
        create_price.expand = &["product"];
        Price::create(&client, create_price).await.unwrap()
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name.unwrap(),
        price.unit_amount.unwrap() / 100,
        price.currency.unwrap()
    );

    let payment_link = PaymentLink::create(
        &client,
        CreatePaymentLink::new(vec![CreatePaymentLinkLineItems {
            quantity: 3,
            price: price.id.to_string(),
            ..Default::default()
        }]),
    )
    .await
    .unwrap();

    println!("created a payment link {}", payment_link.url);
}
