use stripe_client_core::StripeRequest;
use stripe_core::payment_intent::CreatePaymentIntent;
use stripe_core::PaymentIntentSetupFutureUsage;
use stripe_types::Currency;

// Helper: builds the request and returns the raw form-encoded body.
fn wire_body(req: impl StripeRequest) -> String {
    req.build().body.expect("CreatePaymentIntent must have a body")
}

// Parse a form-encoded body into sorted key=value pairs so assertions are
// order-independent (HashMap serialization order is non-deterministic).
fn sorted_pairs(body: &str) -> Vec<(&str, &str)> {
    let mut pairs: Vec<_> = body
        .split('&')
        .filter(|s| !s.is_empty())
        .map(|kv| {
            let (k, v) = kv.split_once('=').expect("malformed pair");
            (k, v)
        })
        .collect();
    pairs.sort();
    pairs
}

#[test]
fn baseline_no_optional_fields() {
    let body = wire_body(CreatePaymentIntent::new(1000i64, Currency::USD));
    let pairs = sorted_pairs(&body);
    assert!(pairs.contains(&("amount", "1000")), "body: {body}");
    assert!(pairs.contains(&("currency", "usd")), "body: {body}");
    // Ensure nothing extra leaks in
    assert_eq!(pairs.len(), 2, "unexpected extra fields in body: {body}");
}

#[test]
fn description_uses_plus_for_spaces() {
    // Stripe's form endpoint expects application/x-www-form-urlencoded where
    // spaces are encoded as '+', not '%20'. Regression against serde_qs RC
    // versions that changed this default.
    let body =
        wire_body(CreatePaymentIntent::new(1000i64, Currency::USD).description("Payment test"));
    assert!(
        body.contains("description=Payment+test"),
        "expected 'Payment+test' (not '%20'), got: {body}"
    );
    assert!(
        !body.contains("description=Payment%20test"),
        "serde_qs is using %20 instead of '+' for spaces: {body}"
    );
}

#[test]
fn metadata_uses_bracket_notation_not_percent_encoded() {
    // Stripe expects metadata[key]=value with literal brackets.
    // serde_qs must NOT percent-encode them as %5B / %5D.
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("order_id".to_string(), "42".to_string());

    let body =
        wire_body(CreatePaymentIntent::new(1000i64, Currency::USD).metadata(metadata));

    assert!(
        body.contains("metadata[order_id]=42"),
        "expected literal brackets 'metadata[order_id]=42', got: {body}"
    );
    assert!(
        !body.contains("metadata%5Border_id%5D"),
        "serde_qs is percent-encoding brackets (%5B/%5D): {body}"
    );
}

#[test]
fn metadata_multiple_keys() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("user_id".to_string(), "usr_99".to_string());
    metadata.insert("internal_ref".to_string(), "ord_abc".to_string());

    let body =
        wire_body(CreatePaymentIntent::new(1000i64, Currency::USD).metadata(metadata));

    assert!(body.contains("metadata[user_id]=usr_99"), "body: {body}");
    assert!(body.contains("metadata[internal_ref]=ord_abc"), "body: {body}");
}

#[test]
fn description_and_metadata_together() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("foo".to_string(), "bar".to_string());

    let body = wire_body(
        CreatePaymentIntent::new(1000i64, Currency::USD)
            .description("Payment test")
            .metadata(metadata),
    );

    assert!(body.contains("description=Payment+test"), "body: {body}");
    assert!(body.contains("metadata[foo]=bar"), "body: {body}");
}

#[test]
fn description_multiword_and_multi_key_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("foo".to_string(), "foo".to_string());
    metadata.insert("bar".to_string(), "bar".to_string());

    let body = wire_body(
        CreatePaymentIntent::new(1000i64, Currency::USD)
            .description("multi word description")
            .metadata(metadata),
    );

    assert!(
        body.contains("description=multi+word+description"),
        "expected spaces encoded as '+', got: {body}"
    );
    assert!(body.contains("metadata[foo]=foo"), "body: {body}");
    assert!(body.contains("metadata[bar]=bar"), "body: {body}");
}

#[test]
fn setup_future_usage_serializes_correctly() {
    let body = wire_body(
        CreatePaymentIntent::new(1000i64, Currency::USD)
            .setup_future_usage(PaymentIntentSetupFutureUsage::OffSession),
    );

    assert!(body.contains("setup_future_usage=off_session"), "body: {body}");
}
