use crate::ids::BalanceTransactionSourceId;
use crate::params::Object;
use crate::resources::BalanceTransactionSource;

impl Object for BalanceTransactionSource {
    type Id = BalanceTransactionSourceId;
    fn id(&self) -> Self::Id {
        use BalanceTransactionSource as Source;
        use BalanceTransactionSourceId as Id;

        match self {
            Source::ApplicationFee(x) => Id::ApplicationFee(x.id()),
            Source::ApplicationFeeRefund(x) => Id::ApplicationFeeRefund(x.id()),
            Source::Charge(x) => Id::Charge(x.id()),
            Source::ConnectCollectionTransfer(_) => Id::None,
            Source::Dispute(x) => Id::Dispute(x.id()),
            Source::IssuingAuthorization(x) => Id::IssuingAuthorization(x.id()),
            Source::IssuingTransaction(x) => Id::IssuingTransaction(x.id()),
            Source::PlatformTaxFee(_) => Id::None,
            Source::Payout(x) => Id::Payout(x.id()),
            Source::Refund(x) => Id::Refund(x.id()),
            Source::ReserveTransaction(_) => Id::None,
            Source::Topup(x) => Id::Topup(x.id()),
            Source::Transfer(x) => Id::Transfer(x.id()),
            Source::TransferReversal(x) => Id::TransferReversal(x.id()),
        }
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
            Source::PlatformTaxFee(x) => x.object(),
            Source::Payout(x) => x.object(),
            Source::Refund(x) => x.object(),
            Source::ReserveTransaction(x) => x.object(),
            Source::Topup(x) => x.object(),
            Source::Transfer(x) => x.object(),
            Source::TransferReversal(x) => x.object(),
        }
    }
}