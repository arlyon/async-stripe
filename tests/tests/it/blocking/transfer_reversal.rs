use stripe_connect::transfer_reversal::CreateIdTransferReversal;

use crate::mock::get_client;

// https://github.com/arlyon/async-stripe/issues/399
#[test]
fn create_transfer_reversal() {
    let client = get_client();

    let mut create = CreateIdTransferReversal::new();
    let id = "tr_Ll53U0VONALFk36".parse().unwrap();
    create.refund_application_fee = Some(true);
    create.amount = Some(4);

    let created = create.send(&client, &id).unwrap();
    assert_eq!(created.amount, 4);
}
