mod mock;

#[test]
fn is_retrievable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let result = stripe::Charge::retrieve(client, &id, &[]);
        let charge = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(charge.id, "ch_123");
        if let Some(cus) = charge.customer {
            assert!(!cus.is_object());
        }
        if let Some(inv) = charge.invoice {
            assert!(!inv.is_object());
        }
    });
}

#[test]
fn is_expandable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let result = stripe::Charge::retrieve(client, &id, &["customer", "invoice"]);
        let charge = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(charge.id, "ch_123");
        if let Some(cus) = charge.customer {
            assert!(cus.is_object());
        }
        if let Some(inv) = charge.invoice {
            assert!(inv.is_object());
        }
    });
}
