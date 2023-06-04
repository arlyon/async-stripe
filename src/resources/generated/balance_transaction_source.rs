/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum BalanceTransactionSource {
    ApplicationFee(crate::application_fee::ApplicationFee),
    Charge(crate::charge::Charge),
    ConnectCollectionTransfer(crate::connect_collection_transfer::ConnectCollectionTransfer),
    Dispute(crate::dispute::Dispute),
    FeeRefund(crate::fee_refund::FeeRefund),
    IssuingAuthorization(crate::issuing::authorization::Authorization),
    IssuingDispute(crate::issuing::dispute::Dispute),
    IssuingTransaction(crate::issuing::transaction::Transaction),
    Payout(crate::payout::Payout),
    PlatformTaxFee(crate::platform_tax_fee::PlatformTaxFee),
    Refund(crate::refund::Refund),
    ReserveTransaction(crate::reserve_transaction::ReserveTransaction),
    TaxDeductedAtSource(crate::tax_deducted_at_source::TaxDeductedAtSource),
    Topup(crate::topup::Topup),
    Transfer(crate::transfer::Transfer),
    TransferReversal(crate::transfer_reversal::TransferReversal),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for BalanceTransactionSource {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;
        let obj_name: crate::deser::ObjectName = from_str(str)?;
        Ok(match obj_name.object.as_str() {
            "application_fee" => Self::ApplicationFee(from_str(str)?),
            "charge" => Self::Charge(from_str(str)?),
            "connect_collection_transfer" => Self::ConnectCollectionTransfer(from_str(str)?),
            "dispute" => Self::Dispute(from_str(str)?),
            "fee_refund" => Self::FeeRefund(from_str(str)?),
            "issuing.authorization" => Self::IssuingAuthorization(from_str(str)?),
            "issuing.dispute" => Self::IssuingDispute(from_str(str)?),
            "issuing.transaction" => Self::IssuingTransaction(from_str(str)?),
            "payout" => Self::Payout(from_str(str)?),
            "platform_tax_fee" => Self::PlatformTaxFee(from_str(str)?),
            "refund" => Self::Refund(from_str(str)?),
            "reserve_transaction" => Self::ReserveTransaction(from_str(str)?),
            "tax_deducted_at_source" => Self::TaxDeductedAtSource(from_str(str)?),
            "topup" => Self::Topup(from_str(str)?),
            "transfer" => Self::Transfer(from_str(str)?),
            "transfer_reversal" => Self::TransferReversal(from_str(str)?),
            _ => return Err(crate::StripeError::JSONDeserialize("Unexpected object name".into())),
        })
    }
}

impl crate::Object for BalanceTransactionSource {
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
