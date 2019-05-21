use crate::config::{Client, Response};
use crate::ids::BalanceTransactionId;
use crate::params::Object;
use crate::resources::{BalanceTransaction, BalanceTransactionSource};

impl BalanceTransaction {
    /// Retrieves the balance transaction with the given ID.
    ///
    /// For more details see [https://stripe.com/docs/api/balance/balance_transaction_retrieve](https://stripe.com/docs/api/balance/balance_transaction_retrieve).
    pub fn retrieve(client: &Client, id: &BalanceTransactionId) -> Response<BalanceTransaction> {
        client.get(&format!("/balance/history/{}", id))
    }
}

type BalanceTransactionSourceId = String; // TODO: Replace with enum

impl Object for BalanceTransactionSource {
    type Id = BalanceTransactionSourceId;
    fn id(&self) -> &Self::Id {
        unimplemented!() // FIXME: Implement!
    }
    fn object(&self) -> &'static str {
        unimplemented!() // FIXME: Implement!
    }
    // ApplicationFee(ApplicationFee),
    // ApplicationFeeRefund(ApplicationFeeRefund),
    // Charge(Charge),
    // ConnectCollectionTransfer(ConnectCollectionTransfer),
    // Dispute(Dispute),
    // IssuingAuthorization(IssuingAuthorization),
    // IssuingTransaction(IssuingTransaction),
    // Payout(Payout),
    // Refund(Refund),
    // ReserveTransaction(ReserveTransaction),
    // Topup(Topup),
    // Transfer(Transfer),
    // TransferReversal(TransferReversal),
}
