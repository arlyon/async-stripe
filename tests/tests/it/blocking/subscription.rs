use stripe_billing::subscription::{
    CancelSubscription, CancelSubscriptionCancellationDetails, RetrieveSubscription,
};

use super::get_client;

#[test]
// Test ignored because the spec implies `plan` is required, but stripe-mock does not
// include it when sending a `subscription_item`
#[ignore]
fn is_subscription_retrievable() {
    let client = get_client();

    let id = "sub_123".parse().unwrap();
    let subscription = RetrieveSubscription::new(&id).send_blocking(&client).unwrap();
    assert_eq!(subscription.id, "sub_123");
    assert!(!subscription.customer.is_object());
}

#[test]
#[ignore]
fn is_subscription_expandable() {
    let client = get_client();

    let id = "sub_123".parse().unwrap();
    let subscription =
        RetrieveSubscription::new(&id).expand(&["customer"]).send_blocking(&client).unwrap();
    assert_eq!(subscription.id, "sub_123");
    assert!(subscription.customer.is_object());
}

#[test]
#[ignore]
/// https://github.com/arlyon/async-stripe/issues/394
/// https://github.com/arlyon/async-stripe/issues/419
fn can_prorate_when_cancelling_subscription() {
    let client = get_client();

    let details = CancelSubscriptionCancellationDetails::new();
    let id = "sub_123".parse().unwrap();
    let result = CancelSubscription::new(&id)
        .prorate(true)
        .cancellation_details(details)
        .send_blocking(&client)
        .unwrap();
    assert_eq!(result.id, id);
}
