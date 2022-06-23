mod mock;

#[test]
#[cfg(feature = "blocking")]
fn is_checkout_session_retrievable() {
    mock::with_client(|client| {
        let id = "cs_test_123".parse().unwrap();
        let result = stripe::CheckoutSession::retrieve(client, &id);
        let session = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(session.id, "cs_test_123");
    });
}
