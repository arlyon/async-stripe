mod mock;

#[test]
#[cfg(feature = "blocking")]
fn is_charge_retrievable() {
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
#[cfg(feature = "blocking")]
fn is_charge_expandable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let result = stripe::Charge::retrieve(
            client,
            &id,
            &[
                "application",
                // "application_fee",
                "balance_transaction",
                "customer",
                "dispute",
                "invoice",
                "review",
                // FIXME: Figure out what the `py_` id prefix is for
                // "source_transfer",
                // "transfer",
            ],
        );
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
