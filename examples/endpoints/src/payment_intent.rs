//! Payment Intent
//! ==============
//!
//! Reference: <https://stripe.com/docs/api/payment_intents>
//!
//! This example shows how to create a [PaymentIntent] and use it to create a
//! charge for a fictional card. We create a customer, an intent, update
//! their payment information, and lastly use the intent to create a charge.

use stripe::{Client, StripeError};
use stripe_core::customer::CreateCustomer;
use stripe_core::payment_intent::{ConfirmPaymentIntent, CreatePaymentIntent, UpdatePaymentIntent};
use stripe_payment::payment_method::{
    AttachPaymentMethod, CreatePaymentMethod, CreatePaymentMethodCard,
    CreatePaymentMethodCardDetailsParams, CreatePaymentMethodType,
};
use stripe_types::Currency;

pub async fn run_payment_intent_example(client: &Client) -> Result<(), StripeError> {
    let customer = CreateCustomer {
        name: Some("Alexander Lyon"),
        email: Some("test@async-stripe.com"),
        description: Some(
            "A fake customer that is used to illustrate the examples in async-stripe.",
        ),
        metadata: Some(&std::collections::HashMap::from([(
            String::from("async-stripe"),
            String::from("true"),
        )])),

        ..Default::default()
    }
    .send(client)
    .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    // we create an intent to pay
    let meta = [("color".to_string(), "red".to_string())].iter().cloned().collect();
    let payment_intent = {
        let mut create_intent = CreatePaymentIntent::new(1000, Currency::USD);
        create_intent.payment_method_types = Some(&["card"]);
        create_intent.statement_descriptor = Some("Purchasing a new car");
        create_intent.metadata = Some(&meta);
        create_intent.send(client).await?
    };

    println!(
        "created a payment intent at https://dashboard.stripe.com/test/payments/{} with status '{}'",
        payment_intent.id, payment_intent.status
    );

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

        AttachPaymentMethod { customer: &customer.id, expand: None }.send(client, &pm.id).await?;

        pm
    };

    println!(
        "created a payment method with id {} and attached it to {}",
        payment_method.id,
        customer.name.unwrap()
    );

    // lets update the payment intent with their details
    let payment_intent = UpdatePaymentIntent {
        payment_method: Some(payment_method.id.as_str()),
        customer: Some(customer.id.as_str()), // this is not strictly required but good practice to ensure we have the right person
        ..Default::default()
    }
    .send(client, &payment_intent.id)
    .await?;

    println!("updated payment intent with status '{}'", payment_intent.status);

    let payment_intent = ConfirmPaymentIntent::new().send(client, &payment_intent.id).await?;
    println!("completed payment intent with status {}", payment_intent.status);
    Ok(())
}
