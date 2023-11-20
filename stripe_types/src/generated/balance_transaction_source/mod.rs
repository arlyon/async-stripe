/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum BalanceTransactionSource {
    #[serde(rename = "application_fee")]
    PlatformFee(stripe_types::PlatformFee),
    #[serde(rename = "charge")]
    Charge(stripe_types::Charge),
    #[serde(rename = "connect_collection_transfer")]
    ConnectCollectionTransfer(stripe_types::ConnectCollectionTransfer),
    #[serde(rename = "customer_cash_balance_transaction")]
    CustomerCashBalanceTransaction(stripe_types::CustomerCashBalanceTransaction),
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
    PlatformTax(stripe_types::PlatformTax),
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
    fn id(&self) -> Option<&str> {
        match self {
            Self::PlatformFee(v) => Some(v.id.as_str()),
            Self::Charge(v) => Some(v.id.as_str()),
            Self::ConnectCollectionTransfer(v) => Some(v.id.as_str()),
            Self::CustomerCashBalanceTransaction(v) => Some(v.id.as_str()),
            Self::Dispute(v) => Some(v.id.as_str()),
            Self::FeeRefund(v) => Some(v.id.as_str()),
            Self::IssuingAuthorization(v) => Some(v.id.as_str()),
            Self::IssuingDispute(v) => Some(v.id.as_str()),
            Self::IssuingTransaction(v) => Some(v.id.as_str()),
            Self::Payout(v) => Some(v.id.as_str()),
            Self::PlatformTax(v) => Some(v.id.as_str()),
            Self::Refund(v) => Some(v.id.as_str()),
            Self::ReserveTransaction(v) => Some(v.id.as_str()),
            Self::TaxDeductedAtSource(v) => Some(v.id.as_str()),
            Self::Topup(v) => Some(v.id.as_str()),
            Self::Transfer(v) => Some(v.id.as_str()),
            Self::TransferReversal(v) => Some(v.id.as_str()),
        }
    }
}
