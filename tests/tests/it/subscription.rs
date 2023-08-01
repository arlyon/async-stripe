use stripe_billing::subscription::requests;
use stripe_billing::subscription::requests::{CancelSubscription, RetrieveSubscription};

use crate::mock;

#[test]
fn is_subscription_retrievable() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let subscription = requests::retrieve(client, &id, RetrieveSubscription::new()).unwrap();
        assert_eq!(subscription.id, "sub_123");
        assert!(!subscription.customer.is_object());
    });
}

#[test]
fn is_subscription_expandable() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let mut params = RetrieveSubscription::new();
        params.expand = Some(&["customer"]);
        let subscription = requests::retrieve(client, &id, params).unwrap();
        assert_eq!(subscription.id, "sub_123");
        assert!(subscription.customer.is_object());
    });
}

#[test]
/// https://github.com/arlyon/async-stripe/issues/394
fn can_prorate_when_cancelling_subscription() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let mut params = CancelSubscription::new();
        params.prorate = Some(true);
        let result = requests::cancel(client, &id, params).unwrap();
        assert_eq!(result.id, id);
    })
}
