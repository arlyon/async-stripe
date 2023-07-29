/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum BalanceTransactionSource {
    ApplicationFee(stripe_types::application_fee::ApplicationFee),
    Charge(stripe_types::charge::Charge),
    ConnectCollectionTransfer(stripe_types::connect_collection_transfer::ConnectCollectionTransfer),
    Dispute(stripe_types::dispute::Dispute),
    FeeRefund(stripe_types::fee_refund::FeeRefund),
    IssuingAuthorization(stripe_types::issuing::authorization::Authorization),
    IssuingDispute(stripe_types::issuing::dispute::Dispute),
    IssuingTransaction(stripe_types::issuing::transaction::Transaction),
    Payout(stripe_types::payout::Payout),
    PlatformTaxFee(stripe_types::platform_tax_fee::PlatformTaxFee),
    Refund(stripe_types::refund::Refund),
    ReserveTransaction(stripe_types::reserve_transaction::ReserveTransaction),
    TaxDeductedAtSource(stripe_types::tax_deducted_at_source::TaxDeductedAtSource),
    Topup(stripe_types::topup::Topup),
    Transfer(stripe_types::transfer::Transfer),
    TransferReversal(stripe_types::transfer_reversal::TransferReversal),
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
