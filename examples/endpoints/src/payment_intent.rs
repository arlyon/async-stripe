//! Payment Intent
//! ==============
//!
//! Reference: <https://stripe.com/docs/api/payment_intents>
//!
//! This example shows how to creeate a [PaymentIntent] and use it to create a
//! charge for a fictional card. We create a customer, an intent, update
//! their payment information, and lastly use the intent to create a charge.

use stripe::{
    AttachPaymentMethod, CardDetailsParams, Client, CreateCustomer, CreatePaymentIntent,
    CreatePaymentMethod, CreatePaymentMethodCardUnion, Currency, Customer, PaymentIntent,
    PaymentIntentConfirmParams, PaymentMethod, PaymentMethodTypeFilter, UpdatePaymentIntent,
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

    // we create an intent to pay
    let payment_intent = {
        let mut create_intent = CreatePaymentIntent::new(1000, Currency::USD);
        create_intent.payment_method_types = Some(vec!["card".to_string()]);
        create_intent.statement_descriptor = Some("Purchasing a new car");
        create_intent.metadata =
            Some([("color".to_string(), "red".to_string())].iter().cloned().collect());

        PaymentIntent::create(&client, create_intent).await.unwrap()
    };

    println!(
        "created a payment intent at https://dashboard.stripe.com/test/payments/{} with status '{}'",
        payment_intent.id, payment_intent.status
    );

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

    // lets update the payment intent with their details
    let payment_intent = PaymentIntent::update(
        &client,
        &payment_intent.id,
        UpdatePaymentIntent {
            payment_method: Some(payment_method.id),
            customer: Some(customer.id), // this is not strictly required but good practice to ensure we have the right person
            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("updated payment intent with status '{}'", payment_intent.status);

    let payment_intent = PaymentIntent::confirm(
        &client,
        &payment_intent.id,
        PaymentIntentConfirmParams { ..Default::default() },
    )
    .await
    .unwrap();

    println!("completed payment intent with status {}", payment_intent.status);
}
