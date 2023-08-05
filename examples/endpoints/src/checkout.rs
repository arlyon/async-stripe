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

use stripe::StripeError;
use stripe_checkout::session::{CreateSession, CreateSessionLineItems, CreateSessionMode};
use stripe_core::customer::CreateCustomer;
use stripe_product::price::CreatePrice;
use stripe_product::product::CreateProduct;
use stripe_types::Expandable;

pub async fn run_checkout_session_example(client: &stripe::Client) -> Result<(), StripeError> {
    let metadata =
        std::collections::HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    let customer = CreateCustomer {
        name: Some("Alexander Lyon"),
        email: Some("test@async-stripe.com"),
        description: Some(
            "A fake customer that is used to illustrate the examples in async-stripe.",
        ),
        metadata: Some(&metadata),
        ..Default::default()
    }
    .send(client)
    .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    // create a new example product
    let product = {
        let mut create_product = CreateProduct::new("T-Shirt");
        create_product.metadata = Some(&metadata);
        create_product.send(client).await?
    };

    // and add a price for it in USD
    let price = {
        let mut create_price = CreatePrice::new(stripe_types::Currency::USD);
        create_price.product = Some(product.id.as_str());
        create_price.metadata = Some(&metadata);
        create_price.unit_amount = Some(1000);
        create_price.expand = Some(&["product"]);
        create_price.send(client).await?
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name,
        price.unit_amount.unwrap() / 100,
        price.currency
    );

    // finally, create a checkout session for this product / price
    let line_items = vec![CreateSessionLineItems {
        quantity: Some(3),
        price: Some(&price.id),
        ..Default::default()
    }];
    let checkout_session = {
        let mut params = CreateSession::new("http://test.com/success");
        params.cancel_url = Some("http://test.com/cancel");
        params.customer = Some(customer.id.as_str());
        params.mode = Some(CreateSessionMode::Payment);
        params.line_items = Some(&line_items);
        params.expand = Some(&["line_items", "line_items.data.price.product"]);
        params.send(client).await?
    };

    println!(
        "created a {} checkout session for {} {:?} for {} {} at {}",
        checkout_session.payment_status,
        checkout_session.line_items.data[0].quantity.unwrap(),
        match &checkout_session.line_items.data[0].price.as_ref().unwrap().product {
            Expandable::Object(p) => &p.name,
            _ => panic!("product not found"),
        },
        checkout_session.amount_subtotal.unwrap() / 100,
        checkout_session.line_items.data[0].price.as_ref().unwrap().currency,
        checkout_session.url.unwrap()
    );
    Ok(())
}
