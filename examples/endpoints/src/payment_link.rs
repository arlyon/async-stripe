//! Payment Link
//! ============
//!
//! Reference: <https://stripe.com/docs/api/payment_links/payment_links>
//!
//! This example shows how to create a payment link for
//! a particular product and price. The nice thing with
//! this API is the lack of associated customer.

use stripe::{Client, StripeError};
use stripe_payment::payment_link::{CreatePaymentLink, CreatePaymentLinkLineItems};
use stripe_product::price::CreatePrice;
use stripe_product::product::CreateProduct;
use stripe_types::Currency;

pub async fn run_payment_link_example(client: &Client) -> Result<(), StripeError> {
    // create a new example project
    let meta =
        std::collections::HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    let product = {
        let mut create_product = CreateProduct::new("T-Shirt");
        create_product.metadata = Some(&meta);
        create_product.send(client).await?
    };

    // and add a price for it in USD
    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(product.id.as_str());
        create_price.metadata = Some(&meta);
        create_price.unit_amount = Some(1000);
        create_price.expand = Some(&["product"]);
        create_price.send(client).await?
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name,
        price.unit_amount.unwrap() / 100,
        price.currency,
    );

    let payment_link = CreatePaymentLink::new(&[CreatePaymentLinkLineItems {
        adjustable_quantity: None,
        quantity: 3,
        price: &price.id,
    }])
    .send(client)
    .await?;

    println!("created a payment link {}", payment_link.url);
    Ok(())
}
