//! Checkout
//! ========
//!
//! Reference: <https://stripe.com/docs/api/checkout/sessions>
//!
//! This example shows how to build a checkout session for
//! a particular product and price. Creating a checkout
//! session generates a URL that the user can use to pay.
//! Notice you have to define the customer ahead of time.
//! If you'd rather avoid this, you can use a [stripe_types::PaymentLink].

use stripe::StripeError;
use stripe_checkout::checkout_session::{CreateCheckoutSession, CreateCheckoutSessionLineItems};
use stripe_checkout::CheckoutSessionMode;
use stripe_core::customer::CreateCustomer;
use stripe_product::price::CreatePrice;
use stripe_product::product::CreateProduct;
use stripe_types::{Currency, Expandable};

pub async fn run_checkout_session_example(client: &stripe::Client) -> Result<(), StripeError> {
    let metadata =
        std::collections::HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    let customer = CreateCustomer::new()
        .name("Alexander Lyon")
        .email("test@async-stripe.com")
        .description("A fake customer that is used to illustrate the examples in async-stripe.")
        .metadata(&metadata)
        .send(client)
        .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    // create a new example product
    let product = CreateProduct::new("T-Shirt").metadata(&metadata).send(client).await?;

    // and add a price for it in USD
    let price = CreatePrice::new(Currency::USD)
        .product(product.id.as_str())
        .metadata(&metadata)
        .unit_amount(1000)
        .expand(&["product"])
        .send(client)
        .await?;

    println!(
        "created a product {:?} at price {} {}",
        product.name,
        price.unit_amount.unwrap() / 100,
        price.currency
    );

    // finally, create a checkout session for this product / price
    let line_items = vec![CreateCheckoutSessionLineItems {
        quantity: Some(3),
        price: Some(&price.id),
        ..Default::default()
    }];
    let checkout_session = CreateCheckoutSession::new()
        .cancel_url("http://test.com/cancel")
        .customer(customer.id.as_str())
        .mode(CheckoutSessionMode::Payment)
        .line_items(&line_items)
        .expand(&["line_items", "line_items.data.price.product"])
        .send(client)
        .await?;

    let created_item = &checkout_session.line_items.expect("line items were created").data[0];
    println!(
        "created a {} checkout session for {} {:?} for {} {} at {}",
        checkout_session.payment_status,
        created_item.quantity.unwrap(),
        match &created_item.price.as_ref().unwrap().product {
            Expandable::Object(p) => &p.name,
            _ => panic!("product not found"),
        },
        checkout_session.amount_subtotal.unwrap() / 100,
        created_item.price.as_ref().unwrap().currency,
        checkout_session.url.unwrap()
    );
    Ok(())
}
