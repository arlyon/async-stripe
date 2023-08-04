use stripe_billing::subscription::{CancelSubscription, RetrieveSubscription};

use crate::mock;

#[test]
// Test ignored because the spec implies `plan` is required, but stripe-mock does not
// include it when sending a `subscription_item`
#[ignore]
fn is_subscription_retrievable() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let subscription = RetrieveSubscription::new().send(client, &id).unwrap();
        assert_eq!(subscription.id, "sub_123");
        assert!(!subscription.customer.is_object());
    });
}

#[test]
#[ignore]
fn is_subscription_expandable() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let mut retrieve = RetrieveSubscription::new();
        retrieve.expand = Some(&["customer"]);
        let subscription = retrieve.send(client, &id).unwrap();
        assert_eq!(subscription.id, "sub_123");
        assert!(subscription.customer.is_object());
    });
}

#[test]
#[ignore]
/// https://github.com/arlyon/async-stripe/issues/394
fn can_prorate_when_cancelling_subscription() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let mut cancel = CancelSubscription::new();
        cancel.prorate = Some(true);
        let result = cancel.send(client, &id).unwrap();
        assert_eq!(result.id, id);
    })
}
