//! Checkout
//! ========
//!
//! Reference: <https://stripe.com/docs/api/checkout/sessions>
//!
//! This example shows how to build a checkout session for
//! a particular product and price. Creating a checkout
//! session generates a URL that the user can use to pay.
//! Notice you have to define the customer ahead of time.
//! If you'd rather avoid this, you can use a [stripe::PaymentLink].

use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCustomer, CreatePrice, CreateProduct, Currency, Customer,
    Expandable, IdOrCreate, Price, Product,
};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let customer = Customer::create(
        &client,
        CreateCustomer {
            name: Some("Alexander Lyon"),
            email: Some("test@async-stripe.com"),
            description: Some(
                "A fake customer that is used to illustrate the examples in async-stripe.",
            ),
            metadata: Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )])),

            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    // create a new example project
    let product = {
        let mut create_product = CreateProduct::new("T-Shirt");
        create_product.metadata = Some(std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));
        Product::create(&client, create_product).await.unwrap()
    };

    // and add a price for it in USD
    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.metadata = Some(std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )]));
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
        let mut params = CreateCheckoutSession::new();
        params.cancel_url = Some("http://test.com/cancel");
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

    let line_items = checkout_session.line_items.unwrap();

    println!(
        "created a {} checkout session for {} {:?} for {} {} at {}",
        checkout_session.payment_status,
        line_items.data[0].quantity.unwrap(),
        match line_items.data[0].price.as_ref().unwrap().product.as_ref().unwrap() {
            Expandable::Object(p) => p.name.as_ref().unwrap(),
            _ => panic!("product not found"),
        },
        checkout_session.amount_subtotal.unwrap() / 100,
        line_items.data[0].price.as_ref().unwrap().currency.unwrap(),
        checkout_session.url.unwrap()
    );
}
