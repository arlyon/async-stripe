use stripe_core::charge::RetrieveCharge;

use crate::mock;

#[test]
fn is_charge_retrievable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let charge = RetrieveCharge::new().send(client, &id).unwrap();
        assert_eq!(charge.id, "ch_123");
        assert!(charge.customer.is_none());
        assert!(charge.invoice.is_none());
        assert_eq!(charge.refunds.data.len(), 1);
    });
}
