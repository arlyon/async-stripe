/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum BalanceTransactionSource {
    #[serde(rename = "application_fee")]
    ApplicationFee(stripe_types::PlatformFee),
    #[serde(rename = "charge")]
    Charge(stripe_types::Charge),
    #[serde(rename = "connect_collection_transfer")]
    ConnectCollectionTransfer(stripe_types::ConnectCollectionTransfer),
    #[serde(rename = "dispute")]
    Dispute(stripe_types::Dispute),
    #[serde(rename = "fee_refund")]
    FeeRefund(stripe_types::FeeRefund),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(stripe_types::IssuingAuthorization),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(stripe_types::IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(stripe_types::IssuingTransaction),
    #[serde(rename = "payout")]
    Payout(stripe_types::Payout),
    #[serde(rename = "platform_tax_fee")]
    PlatformTaxFee(stripe_types::PlatformTax),
    #[serde(rename = "refund")]
    Refund(stripe_types::Refund),
    #[serde(rename = "reserve_transaction")]
    ReserveTransaction(stripe_types::ReserveTransaction),
    #[serde(rename = "tax_deducted_at_source")]
    TaxDeductedAtSource(stripe_types::TaxDeductedAtSource),
    #[serde(rename = "topup")]
    Topup(stripe_types::Topup),
    #[serde(rename = "transfer")]
    Transfer(stripe_types::Transfer),
    #[serde(rename = "transfer_reversal")]
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
