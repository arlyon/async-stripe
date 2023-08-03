/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BalanceTransactionSource {
    ApplicationFee(stripe_types::PlatformFee),
    Charge(stripe_types::Charge),
    ConnectCollectionTransfer(stripe_types::ConnectCollectionTransfer),
    Dispute(stripe_types::Dispute),
    FeeRefund(stripe_types::FeeRefund),
    IssuingAuthorization(stripe_types::IssuingAuthorization),
    IssuingDispute(stripe_types::IssuingDispute),
    IssuingTransaction(stripe_types::IssuingTransaction),
    Payout(stripe_types::Payout),
    PlatformTaxFee(stripe_types::PlatformTax),
    Refund(stripe_types::Refund),
    ReserveTransaction(stripe_types::ReserveTransaction),
    TaxDeductedAtSource(stripe_types::TaxDeductedAtSource),
    Topup(stripe_types::Topup),
    Transfer(stripe_types::Transfer),
    TransferReversal(stripe_types::TransferReversal),
}
impl stripe_types::Object for BalanceTransactionSource {
    type Id = String;
    fn id(&self) -> Self::Id {
        match self {
            Self::ApplicationFee(v) => v.id.to_string(),
            Self::Charge(v) => v.id.to_string(),
            Self::ConnectCollectionTransfer(v) => v.id.to_string(),
            Self::Dispute(v) => v.id.to_string(),
            Self::FeeRefund(v) => v.id.to_string(),
            Self::IssuingAuthorization(v) => v.id.to_string(),
            Self::IssuingDispute(v) => v.id.to_string(),
            Self::IssuingTransaction(v) => v.id.to_string(),
            Self::Payout(v) => v.id.to_string(),
            Self::PlatformTaxFee(v) => v.id.to_string(),
            Self::Refund(v) => v.id.to_string(),
            Self::ReserveTransaction(v) => v.id.to_string(),
            Self::TaxDeductedAtSource(v) => v.id.to_string(),
            Self::Topup(v) => v.id.to_string(),
            Self::Transfer(v) => v.id.to_string(),
            Self::TransferReversal(v) => v.id.to_string(),
        }
    }
}
