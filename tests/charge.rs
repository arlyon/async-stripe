mod mock;

#[test]
fn is_retrievable() {
    mock::with_client(|client| {
        let result = stripe::Charge::retrieve(client, "ch_123");
        let charge = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(charge.id, "ch_123");
    });
}
