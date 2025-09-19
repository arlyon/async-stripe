use stripe_billing::{
    InvoiceId,
    invoice::{FinalizeInvoiceInvoice, PayInvoice, RetrieveInvoice},
};

use super::get_client;

// Smoke test, https://github.com/arlyon/async-stripe/issues/396
#[test]
fn is_invoice_retrievable() {
    let client = get_client();
    let id = InvoiceId::from("in_123");
    let result = RetrieveInvoice::new(id.clone())
        .expand([String::from("charge.balance_transaction")])
        .send_blocking(&client)
        .unwrap();
    assert_eq!(result.id, Some(id));
}

// https://github.com/arlyon/async-stripe/issues/446
// https://github.com/arlyon/async-stripe/issues/352
#[test]
fn is_invoice_payable() {
    let client = get_client();

    let id = InvoiceId::from("in_123");

    let result = PayInvoice::new(&id)
        .forgive(true)
        .off_session(true)
        .paid_out_of_band(true)
        .send_blocking(&client)
        .unwrap();
    assert_eq!(result.id, Some(id));
    assert_eq!(result.amount_overpaid, 149300825);
    assert_eq!(result.amount_paid, 0);
}

#[test]
// https://github.com/arlyon/async-stripe/issues/442
fn finalize_invoice() {
    let client = get_client();

    let id = "in_123".parse().unwrap();
    let result =
        FinalizeInvoiceInvoice::new(&id).auto_advance(true).send_blocking(&client).unwrap();
    assert_eq!(result.id, Some(id));
    assert_eq!(result.auto_advance, Some(true));
}
