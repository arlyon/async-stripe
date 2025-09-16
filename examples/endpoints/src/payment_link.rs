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
    // create a new example product
    let product = CreateProduct::new("T-Shirt")
        .metadata([(String::from("async-stripe"), String::from("true"))])
        .send(client)
        .await?;

    // and add a price for it in USD
    let price = CreatePrice::new(Currency::USD)
        .product(product.id.as_str())
        .metadata([(String::from("async-stripe"), String::from("true"))])
        .unit_amount(1000)
        .expand([String::from("product")])
        .send(client)
        .await?;

    println!(
        "created a product {:?} at price {} {}",
        product.name,
        price.unit_amount.unwrap() / 100,
        price.currency,
    );

    let payment_link = CreatePaymentLink::new(&[CreatePaymentLinkLineItems {
        adjustable_quantity: None,
        quantity: 3,
        price: Some(price.id.to_string()),
        price_data: None,
    }])
    .send(client)
    .await?;

    println!("created a payment link {}", payment_link.url);
    Ok(())
}
