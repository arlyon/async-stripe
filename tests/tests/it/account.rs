use stripe_connect::account::requests::ListAccount;

use crate::mock;

#[test]
fn is_account_listable() {
    mock::with_client(|client| {
        let result = stripe_connect::account::requests::list(client, ListAccount::new()).unwrap();
        // Check to ensure we are deserializing _something_ and this test
        // actually validates something worthwhile.
        assert!(result.data.len() > 0);
    });
}
