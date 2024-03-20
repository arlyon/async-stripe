use stripe_billing::subscription::{
    CancelSubscription, CancelSubscriptionCancellationDetails, RetrieveSubscription,
};

use crate::mock::get_client;

#[test]
// Test ignored because the spec implies `plan` is required, but stripe-mock does not
// include it when sending a `subscription_item`
#[ignore]
fn is_subscription_retrievable() {
    let client = get_client();

    let id = "sub_123".parse().unwrap();
    let subscription = RetrieveSubscription::new().send(&client, &id).unwrap();
    assert_eq!(subscription.id, "sub_123");
    assert!(!subscription.customer.is_object());
}

#[test]
#[ignore]
fn is_subscription_expandable() {
    let client = get_client();

    let id = "sub_123".parse().unwrap();
    let mut retrieve = RetrieveSubscription::new();
    retrieve.expand = Some(&["customer"]);
    let subscription = retrieve.send(&client, &id).unwrap();
    assert_eq!(subscription.id, "sub_123");
    assert!(subscription.customer.is_object());
}

#[test]
#[ignore]
/// https://github.com/arlyon/async-stripe/issues/394
/// https://github.com/arlyon/async-stripe/issues/419
fn can_prorate_when_cancelling_subscription() {
    let client = get_client();

    let id = "sub_123".parse().unwrap();
    let mut cancel = CancelSubscription::new();
    cancel.cancellation_details = Some(CancelSubscriptionCancellationDetails::new());
    cancel.prorate = Some(true);
    let result = cancel.send(&client, &id).unwrap();
    assert_eq!(result.id, id);
}
