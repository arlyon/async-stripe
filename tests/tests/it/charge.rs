use stripe_core::charge;
use stripe_core::charge::requests::RetrieveCharge;

use crate::mock;

#[test]
fn is_charge_retrievable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let charge = charge::requests::retrieve(client, &id, RetrieveCharge::new()).unwrap();
        assert_eq!(charge.id, "ch_123");
        assert!(charge.customer.is_none());
        assert!(charge.invoice.is_none());
        assert_eq!(charge.refunds.data.len(), 1);
    });
}
