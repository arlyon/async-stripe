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

impl Object for BalanceTransactionSource {
    type Id = ();
    fn id(&self) -> &Self::Id {
        &() // TODO: Some patterns contain _do_ contain ids
    }
    fn object(&self) -> &'static str {
        use BalanceTransactionSource as Source;

        match self {
            Source::ApplicationFee(x) => x.object(),
            Source::ApplicationFeeRefund(x) => x.object(),
            Source::Charge(x) => x.object(),
            Source::ConnectCollectionTransfer(x) => x.object(),
            Source::Dispute(x) => x.object(),
            Source::IssuingAuthorization(x) => x.object(),
            Source::IssuingTransaction(x) => x.object(),
            Source::Payout(x) => x.object(),
            Source::Refund(x) => x.object(),
            Source::ReserveTransaction(x) => x.object(),
            Source::Topup(x) => x.object(),
            Source::Transfer(x) => x.object(),
            Source::TransferReversal(x) => x.object(),
        }
    }
}
