/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl stripe_types::StripeDeserialize for BalanceTransactionSource {
    fn deserialize(str: &str) -> Result<Self, String> {
        use miniserde::json::from_str;
        use stripe_types::StripeDeserialize;
        let obj_name: stripe_types::deser::ObjectName =
            from_str(str).map_err(|_| "Missing `object` field")?;
        Ok(match obj_name.object.as_str() {
            "application_fee" => Self::ApplicationFee(StripeDeserialize::deserialize(str)?),
            "charge" => Self::Charge(StripeDeserialize::deserialize(str)?),
            "connect_collection_transfer" => {
                Self::ConnectCollectionTransfer(StripeDeserialize::deserialize(str)?)
            }
            "dispute" => Self::Dispute(StripeDeserialize::deserialize(str)?),
            "fee_refund" => Self::FeeRefund(StripeDeserialize::deserialize(str)?),
            "issuing.authorization" => {
                Self::IssuingAuthorization(StripeDeserialize::deserialize(str)?)
            }
            "issuing.dispute" => Self::IssuingDispute(StripeDeserialize::deserialize(str)?),
            "issuing.transaction" => Self::IssuingTransaction(StripeDeserialize::deserialize(str)?),
            "payout" => Self::Payout(StripeDeserialize::deserialize(str)?),
            "platform_tax_fee" => Self::PlatformTaxFee(StripeDeserialize::deserialize(str)?),
            "refund" => Self::Refund(StripeDeserialize::deserialize(str)?),
            "reserve_transaction" => Self::ReserveTransaction(StripeDeserialize::deserialize(str)?),
            "tax_deducted_at_source" => {
                Self::TaxDeductedAtSource(StripeDeserialize::deserialize(str)?)
            }
            "topup" => Self::Topup(StripeDeserialize::deserialize(str)?),
            "transfer" => Self::Transfer(StripeDeserialize::deserialize(str)?),
            "transfer_reversal" => Self::TransferReversal(StripeDeserialize::deserialize(str)?),
            _ => return Err("Unexpected object name".into()),
        })
    }
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
