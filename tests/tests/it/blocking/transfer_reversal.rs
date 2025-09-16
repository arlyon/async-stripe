use stripe_connect::{transfer_reversal::CreateIdTransferReversal, TransferId};

use super::get_client;

// https://github.com/arlyon/async-stripe/issues/399
#[test]
fn create_transfer_reversal() {
    let client = get_client();

    let id = TransferId::from("tr_Ll53U0VONALFk36");
    let created = CreateIdTransferReversal::new(id)
        .refund_application_fee(true)
        .amount(4)
        .send_blocking(&client)
        .unwrap();
    assert_eq!(created.amount, 4);
}
