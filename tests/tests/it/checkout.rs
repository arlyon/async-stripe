use stripe_checkout::checkout::session::requests;
use stripe_checkout::checkout::session::requests::RetrieveSession;

use crate::mock;

#[test]
fn is_checkout_session_retrievable() {
    mock::with_client(|client| {
        let id = "cs_test_123".parse().unwrap();
        let session = requests::retrieve(client, &id, RetrieveSession::new()).unwrap();
        assert_eq!(session.id, "cs_test_123");
    });
}
