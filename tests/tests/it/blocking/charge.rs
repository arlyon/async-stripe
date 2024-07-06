use stripe_core::charge::RetrieveCharge;

#[test]
fn is_charge_retrievable() {
    let client = super::get_client();

    let id = "ch_123".parse().unwrap();
    let charge = RetrieveCharge::new(&id).send_blocking(&client).unwrap();
    assert_eq!(charge.id, "ch_123");
    assert!(charge.customer.is_none());
    assert!(charge.invoice.is_none());
    assert_eq!(charge.refunds.unwrap().data.len(), 1);
}
