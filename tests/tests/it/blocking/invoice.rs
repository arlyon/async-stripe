use stripe_billing::invoice::{
    FinalizeInvoiceInvoice, PayInvoice, RetrieveInvoice, UpcomingInvoice,
    UpcomingInvoiceSubscriptionItems,
};

use crate::mock::get_client;

// Smoke test, https://github.com/arlyon/async-stripe/issues/396
#[test]
fn is_invoice_retrievable() {
    let client = get_client();

    let mut retriever = RetrieveInvoice::new();
    retriever.expand = Some(&["charge.balance_transaction"]);
    let id = "in_123".parse().unwrap();
    let result = retriever.send(&client, &id).unwrap();
    let charge = result.charge.unwrap();
    assert!(charge.is_object());
}

// https://github.com/arlyon/async-stripe/issues/446
// https://github.com/arlyon/async-stripe/issues/352
#[test]
fn is_invoice_payable() {
    let client = get_client();

    let mut payer = PayInvoice::new();
    payer.forgive = Some(true);
    payer.off_session = Some(true);
    payer.paid_out_of_band = Some(true);
    let id = "in_123".parse().unwrap();
    let result = payer.send(&client, &id).unwrap();
    assert_eq!(result.id, Some(id));
    assert_eq!(result.paid_out_of_band, true);
}

#[test]
// https://github.com/arlyon/async-stripe/issues/442
fn finalize_invoice() {
    let client = get_client();

    let mut finalize = FinalizeInvoiceInvoice::new();
    finalize.auto_advance = Some(true);
    let id = "in_123".parse().unwrap();
    let result = finalize.send(&client, &id).unwrap();
    assert_eq!(result.id, Some(id));
    assert_eq!(result.auto_advance, Some(true));
}

#[test]
// https://github.com/arlyon/async-stripe/blob/ca5269ebcf9cbd7005f3fecedc63cc31718680a6/src/resources/invoice_ext.rs#L35
fn upcoming_invoice() {
    let client = get_client();

    let mut upcoming = UpcomingInvoice::new();
    let items = vec![UpcomingInvoiceSubscriptionItems::new()];
    upcoming.subscription_items = Some(&items);
    let result = upcoming.send(&client).unwrap();
    assert_eq!(result.subtotal, 1000);
    assert_eq!(result.amount_due, 1000);
}
