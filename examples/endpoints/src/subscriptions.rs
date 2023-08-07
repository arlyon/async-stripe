//! Subscription
//! ============
//!
//! Reference: <https://stripe.com/docs/api/subscriptions>
//!
//! This example shows how to create a subscription for
//! a particular product and recurring price.

use stripe::{Client, StripeError};
use stripe_billing::subscription::{CreateSubscription, CreateSubscriptionItems};
use stripe_core::customer::CreateCustomer;
use stripe_payment::payment_method::{
    AttachPaymentMethod, CreatePaymentMethod, CreatePaymentMethodCard,
    CreatePaymentMethodCardDetailsParams, CreatePaymentMethodType,
};
use stripe_product::price::{CreatePrice, CreatePriceRecurring, Interval};
use stripe_product::product::CreateProduct;
use stripe_types::{Currency, Expandable};

pub async fn run_subscriptions_example(client: &Client) -> Result<(), StripeError> {
    let meta =
        std::collections::HashMap::from([(String::from("async-stripe"), String::from("true"))]);
    let customer = CreateCustomer {
        name: Some("Alexander Lyon"),
        email: Some("test@async-stripe.com"),
        description: Some(
            "A fake customer that is used to illustrate the examples in async-stripe.",
        ),
        metadata: Some(&meta),
        ..Default::default()
    }
    .send(client)
    .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    let payment_method = {
        let pm = CreatePaymentMethod {
            type_: Some(CreatePaymentMethodType::Card),
            card: Some(CreatePaymentMethodCard::CardDetailsParams(
                CreatePaymentMethodCardDetailsParams {
                    number: "4000008260000000", // UK visa
                    exp_year: 2025,
                    exp_month: 1,
                    cvc: Some("123"),
                },
            )),
            ..Default::default()
        }
        .send(client)
        .await?;

        AttachPaymentMethod::new(&customer.id).send(client, &pm.id).await?;
        pm
    };

    println!(
        "created a payment method with id {} and attached it to {}",
        payment_method.id,
        customer.name.unwrap()
    );

    // create a new example product
    let product = {
        let mut create_product = CreateProduct::new("Monthly T-Shirt Subscription");
        create_product.metadata = Some(&meta);
        create_product.send(client).await?
    };

    // and add a price for it in USD
    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(&product.id);
        create_price.metadata = Some(&meta);
        create_price.unit_amount = Some(1000);
        create_price.recurring = Some(CreatePriceRecurring::new(Interval::Month));
        create_price.expand = Some(&["product"]);
        create_price.send(client).await?
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name,
        price.unit_amount.unwrap() / 100,
        price.currency,
    );

    let create_items = [CreateSubscriptionItems { price: Some(&price.id), ..Default::default() }];
    let subscription = {
        let mut params = CreateSubscription::new(&customer.id);
        params.items = Some(&create_items);
        params.default_payment_method = Some(&payment_method.id);
        params.expand = Some(&["items", "items.data.price.product", "schedule"]);
        params.send(client).await?
    };

    println!(
        "created a {} subscription for {:?} for {} {} per {} at https://dashboard.stripe.com/test/subscriptions/{}",
        subscription.status,
        match &subscription.items.data[0].price.product {
            Expandable::Object(p) => &p.name,
            _ => panic!("product not found"),
        },
        subscription.items.data[0].price.unit_amount.unwrap() / 100,
        subscription.items.data[0].price.currency,
        subscription.items.data[0].price.recurring.as_ref().unwrap().interval,
        subscription.id
    );
    Ok(())
}
