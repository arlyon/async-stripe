//! Payment Intent
//! ==============
//!
//! Reference: <https://stripe.com/docs/api/payment_intents>
//!
//! This example shows how to create a [stripe_types::PaymentIntent] and use it to create a
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
    let customer = CreateCustomer::new()
        .name("Alexander Lyon")
        .email("test@async-stripe.com")
        .description("A fake customer that is used to illustrate the examples in async-stripe.")
        .metadata([(String::from("async-stripe"), String::from("true"))])
        .send(client)
        .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);

    // we create an intent to pay
    let payment_intent = CreatePaymentIntent::new(1000, Currency::USD)
        .payment_method_types([String::from("card")])
        .statement_descriptor("Purchasing a new car")
        .metadata([("color".to_string(), "red".to_string())])
        .send(client)
        .await?;

    println!(
        "created a payment intent at https://dashboard.stripe.com/test/payments/{} with status '{}'",
        payment_intent.id, payment_intent.status
    );

    let payment_method = CreatePaymentMethod::new()
        .type_(CreatePaymentMethodType::Card)
        .card(CreatePaymentMethodCard::CardDetailsParams(CreatePaymentMethodCardDetailsParams {
            number: String::from("4000008260000000"), // UK visa
            exp_year: 2025,
            exp_month: 1,
            cvc: Some(String::from("123")),
            networks: None,
        }))
        .send(client)
        .await?;

    AttachPaymentMethod::new(payment_method.id.clone()).customer(&customer.id).send(client).await?;

    println!(
        "created a payment method with id {} and attached it to {}",
        payment_method.id,
        customer.name.unwrap()
    );

    // lets update the payment intent with their details
    let payment_intent = UpdatePaymentIntent::new(payment_intent.id)
        .payment_method(payment_method.id.as_str())
        // this is not strictly required but good practice to ensure we have the right person
        .customer(customer.id.as_str())
        .send(client)
        .await?;

    println!("updated payment intent with status '{}'", payment_intent.status);

    let payment_intent = ConfirmPaymentIntent::new(payment_intent.id).send(client).await?;
    println!("completed payment intent with status {}", payment_intent.status);
    Ok(())
}
