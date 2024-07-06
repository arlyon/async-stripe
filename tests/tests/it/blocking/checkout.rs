use stripe_checkout::checkout_session::RetrieveCheckoutSession;

use super::get_client;

#[test]
fn is_checkout_session_retrievable() {
    let client = get_client();

    let id = "cs_test_123".parse().unwrap();
    let session = RetrieveCheckoutSession::new(&id).send_blocking(&client).unwrap();
    assert_eq!(session.id, "cs_test_123");
}
