use stripe_core::payment_intent::CreatePaymentIntent;
use stripe_types::Currency;

use super::get_client;

#[test]
fn create_payment_intent_with_description_and_metadata() {
    let client = get_client();

    let mut metadata = std::collections::HashMap::new();
    metadata.insert("foo".to_string(), "foo".to_string());
    metadata.insert("bar".to_string(), "bar".to_string());

    let pi = CreatePaymentIntent::new(1000i64, Currency::USD)
        .description("multi word description")
        .metadata(metadata)
        .send_blocking(&client)
        .expect("failed to create payment intent");

    assert_eq!(pi.amount, 1000);
    assert_eq!(pi.currency, Currency::USD);
}
