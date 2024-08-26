use stripe_billing::{
    invoice::{
        FinalizeInvoiceInvoice, PayInvoice, RetrieveInvoice, UpcomingInvoice,
        UpcomingInvoiceSubscriptionItems,
    },
    InvoiceId,
};
use stripe_core::ChargeId;

use super::get_client;

// Smoke test, https://github.com/arlyon/async-stripe/issues/396
#[test]
fn is_invoice_retrievable() {
    let client = get_client();
    let id = InvoiceId::from("in_123");
    let result = RetrieveInvoice::new(id)
        .expand([String::from("charge.balance_transaction")])
        .send_blocking(&client)
        .unwrap();
    let charge = result.charge.unwrap();
    let expected_id = ChargeId::from("ch_1OPoueJN5vQBdWEx5AWizjSY".to_string());
    assert!(charge.is_object());
    assert_eq!(charge.id(), &expected_id);
    assert_eq!(charge.into_id(), expected_id);
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
    assert!(result.paid_out_of_band);
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

#[test]
// https://github.com/arlyon/async-stripe/blob/ca5269ebcf9cbd7005f3fecedc63cc31718680a6/src/resources/invoice_ext.rs#L35
fn upcoming_invoice() {
    let client = get_client();

    let result = UpcomingInvoice::new()
        .subscription_items(&[UpcomingInvoiceSubscriptionItems::new()])
        .send_blocking(&client)
        .unwrap();
    assert_eq!(result.subtotal, 1000);
    assert_eq!(result.amount_due, 1000);
}
