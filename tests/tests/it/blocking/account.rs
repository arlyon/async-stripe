use stripe_connect::account::{CreateAccount, ListAccount};
use stripe_types::AccountId;

use crate::mock;

#[test]
fn is_account_listable() {
    mock::with_client(|client| {
        let expected_id: AccountId = "acct_1NTAy9JtuVGV42eh".parse().unwrap();
        let result = ListAccount::new().send(client).unwrap();

        // Check to ensure we are deserializing _something_ and this test
        // actually validates something worthwhile.
        assert_eq!(result.data.len(), 1);
        assert_eq!(result.data.first().unwrap().id, expected_id);

        let result = ListAccount::new().paginate().get_all(client).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().id, expected_id);
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
