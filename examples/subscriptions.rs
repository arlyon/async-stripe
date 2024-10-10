//! Subscription
//! ============
//!
//! Reference: <https://stripe.com/docs/api/subscriptions>
//!
//! This example shows how to create a subscription for
//! a particular product and recurring price.

use stripe::{
    AttachPaymentMethod, CardDetailsParams, Client, CreateCustomer, CreatePaymentMethod,
    CreatePaymentMethodCardUnion, CreatePrice, CreatePriceRecurring, CreatePriceRecurringInterval,
    CreateProduct, CreateSubscription, CreateSubscriptionItems, Currency, Customer, Expandable,
    IdOrCreate, PaymentMethod, PaymentMethodTypeFilter, Price, Product, Subscription,
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

    let payment_method = {
        let pm = PaymentMethod::create(
            &client,
            CreatePaymentMethod {
                type_: Some(PaymentMethodTypeFilter::Card),
                card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(CardDetailsParams {
                    number: "4000008260000000".to_string(), // UK visa
                    exp_year: 2025,
                    exp_month: 1,
                    cvc: Some("123".to_string()),
                    ..Default::default()
                })),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        PaymentMethod::attach(
            &client,
            &pm.id,
            AttachPaymentMethod { customer: customer.id.clone() },
        )
        .await
        .unwrap();

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
        create_price.recurring = Some(CreatePriceRecurring {
            interval: CreatePriceRecurringInterval::Month,
            ..Default::default()
        });
        create_price.expand = &["product"];
        Price::create(&client, create_price).await.unwrap()
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name.unwrap(),
        price.unit_amount.unwrap() / 100,
        price.currency.unwrap()
    );

    let subscription = {
        let mut params = CreateSubscription::new(customer.id);
        params.items = Some(vec![CreateSubscriptionItems {
            price: Some(price.id.to_string()),
            ..Default::default()
        }]);
        params.default_payment_method = Some(&payment_method.id);
        params.expand = &["items", "items.data.price.product", "schedule"];

        Subscription::create(&client, params).await.unwrap()
    };

    println!(
        "created a {} subscription for {:?} for {} {} per {} at https://dashboard.stripe.com/test/subscriptions/{}",
        subscription.status,
        match subscription.items.data[0].price.as_ref().unwrap().product.as_ref().unwrap() {
            Expandable::Object(p) => p.name.as_ref().unwrap(),
            _ => panic!("product not found"),
        },
        subscription.items.data[0].price.as_ref().unwrap().unit_amount.unwrap() / 100,
        subscription.items.data[0].price.as_ref().unwrap().currency.unwrap(),
        subscription.items.data[0].price.as_ref().unwrap().recurring.as_ref().unwrap().interval,
        subscription.id
    );
}
