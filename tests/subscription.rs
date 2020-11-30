mod mock;

#[test]
#[cfg(feature = "blocking")]
fn is_subscription_retrievable() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let result = stripe::Subscription::retrieve(client, &id, &[]);
        let subscription = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(subscription.id, "sub_123");
        assert!(!subscription.customer.is_object());
    });
}

#[test]
#[cfg(feature = "blocking")]
fn is_subscription_expandable() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let result = stripe::Subscription::retrieve(
            client,
            &id,
            &[
                "customer",
                "schedule",
                "latest_invoice",
                "pending_setup_intent",
                "default_source",
                "default_tax_rates",
                "default_payment_method",
            ],
        );
        let subscription = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(subscription.id, "sub_123");
        assert!(subscription.customer.is_object());
    });
}
