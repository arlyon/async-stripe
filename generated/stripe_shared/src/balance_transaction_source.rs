/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum BalanceTransactionSource {
    #[serde(rename = "application_fee")]
    ApplicationFee(stripe_shared::ApplicationFee),
    #[serde(rename = "charge")]
    Charge(stripe_shared::Charge),
    #[serde(rename = "connect_collection_transfer")]
    ConnectCollectionTransfer(stripe_shared::ConnectCollectionTransfer),
    #[serde(rename = "customer_cash_balance_transaction")]
    CustomerCashBalanceTransaction(stripe_shared::CustomerCashBalanceTransaction),
    #[serde(rename = "dispute")]
    Dispute(stripe_shared::Dispute),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(stripe_shared::ApplicationFeeRefund),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(stripe_shared::IssuingAuthorization),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(stripe_shared::IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(stripe_shared::IssuingTransaction),
    #[serde(rename = "payout")]
    Payout(stripe_shared::Payout),
    #[serde(rename = "platform_tax_fee")]
    PlatformTaxFee(stripe_shared::PlatformTaxFee),
    #[serde(rename = "refund")]
    Refund(stripe_shared::Refund),
    #[serde(rename = "reserve_transaction")]
    ReserveTransaction(stripe_shared::ReserveTransaction),
    #[serde(rename = "tax_deducted_at_source")]
    TaxDeductedAtSource(stripe_shared::TaxDeductedAtSource),
    #[serde(rename = "topup")]
    Topup(stripe_shared::Topup),
    #[serde(rename = "transfer")]
    Transfer(stripe_shared::Transfer),
    #[serde(rename = "transfer_reversal")]
    TransferReversal(stripe_shared::TransferReversal),
}
impl stripe_types::Object for BalanceTransactionSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::ApplicationFee(v) => v.id.inner(),
            Self::Charge(v) => v.id.inner(),
            Self::ConnectCollectionTransfer(v) => v.id.inner(),
            Self::CustomerCashBalanceTransaction(v) => v.id.inner(),
            Self::Dispute(v) => v.id.inner(),
            Self::ApplicationFeeRefund(v) => v.id.inner(),
            Self::IssuingAuthorization(v) => v.id.inner(),
            Self::IssuingDispute(v) => v.id.inner(),
            Self::IssuingTransaction(v) => v.id.inner(),
            Self::Payout(v) => v.id.inner(),
            Self::PlatformTaxFee(v) => v.id.inner(),
            Self::Refund(v) => v.id.inner(),
            Self::ReserveTransaction(v) => v.id.inner(),
            Self::TaxDeductedAtSource(v) => v.id.inner(),
            Self::Topup(v) => v.id.inner(),
            Self::Transfer(v) => v.id.inner(),
            Self::TransferReversal(v) => v.id.inner(),
        }
    }
}
