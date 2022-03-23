//! Checkout
//! ========
//!
//! This example shows the stripe checkout session api.
//! See more: https://stripe.com/docs/api/checkout/sessions

use common::create_customer;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, Customer, Expandable,
    IdOrCreate, Price, Product,
};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let customer = create_customer(&client).await;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

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

    // finally, create a checkout session for this product / price
    let checkout_session = {
        let mut params =
            CreateCheckoutSession::new("http://test.com/cancel", "http://test.com/success");
        params.customer = Some(customer.id);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(3),
            price: Some(price.id.to_string()),
            ..Default::default()
        }]);
        params.expand = &["line_items", "line_items.data.price.product"];

        CheckoutSession::create(&client, params).await.unwrap()
    };

    println!(
        "created a {} checkout session for {} {:?} for {} {} at {}",
        checkout_session.payment_status,
        checkout_session.line_items.data[0].quantity.unwrap(),
        match checkout_session.line_items.data[0].price.as_ref().unwrap().product.as_ref().unwrap()
        {
            Expandable::Object(p) => p.name.as_ref().unwrap(),
            _ => panic!("product not found"),
        },
        checkout_session.amount_subtotal.unwrap() / 100,
        checkout_session.line_items.data[0].price.as_ref().unwrap().currency.unwrap(),
        checkout_session.url.unwrap()
    );
}
