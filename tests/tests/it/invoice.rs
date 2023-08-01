// Smoke test, https://github.com/arlyon/async-stripe/issues/396

use stripe_billing::invoice::requests::RetrieveInvoice;

use crate::mock;

#[test]
fn is_invoice_retrievable() {
    mock::with_client(|client| {
        let mut params = RetrieveInvoice::new();
        params.expand = Some(&["charge.balance_transaction"]);
        let id = "in_123".parse().unwrap();
        let result = stripe_billing::invoice::requests::retrieve(client, &id, params).unwrap();
        let charge = result.charge.unwrap();
        assert!(charge.is_object());
    });
}
