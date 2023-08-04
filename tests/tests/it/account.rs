use stripe_connect::account::{CreateAccount, ListAccount};

use crate::mock;

#[test]
fn is_account_listable() {
    mock::with_client(|client| {
        let result = ListAccount::new().send(client).unwrap();
        // Check to ensure we are deserializing _something_ and this test
        // actually validates something worthwhile.
        assert!(result.data.len() > 0);
    });
}

#[test]
fn create_account() {
    mock::with_client(|client| {
        let create = CreateAccount::new();
        let result = create.send(client).unwrap();
        assert_eq!(result.email, Some("site@stripe.com".to_string()));
    });
}
