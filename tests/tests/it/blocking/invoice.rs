use stripe_billing::invoice::{PayInvoice, RetrieveInvoice};

use crate::mock;

// Smoke test, https://github.com/arlyon/async-stripe/issues/396
#[test]
fn is_invoice_retrievable() {
    mock::with_client(|client| {
        let mut retriever = RetrieveInvoice::new();
        retriever.expand = Some(&["charge.balance_transaction"]);
        let id = "in_123".parse().unwrap();
        let result = retriever.send(client, &id).unwrap();
        let charge = result.charge.unwrap();
        assert!(charge.is_object());
    });
}

// https://github.com/arlyon/async-stripe/issues/446
#[test]
fn is_invoice_payable() {
    mock::with_client(|client| {
        let mut payer = PayInvoice::new();
        payer.forgive = Some(true);
        payer.off_session = Some(true);
        let id = "in_123".parse().unwrap();
        let result = payer.send(client, &id).unwrap();
        assert_eq!(result.id, Some(id));
    })
}
