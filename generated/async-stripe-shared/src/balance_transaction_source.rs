/// The resource representing a Stripe Polymorphic
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum BalanceTransactionSource {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "application_fee")
    )]
    ApplicationFee(stripe_shared::ApplicationFee),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "charge"))]
    Charge(stripe_shared::Charge),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "connect_collection_transfer")
    )]
    ConnectCollectionTransfer(stripe_shared::ConnectCollectionTransfer),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "customer_cash_balance_transaction")
    )]
    CustomerCashBalanceTransaction(stripe_shared::CustomerCashBalanceTransaction),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "dispute"))]
    Dispute(stripe_shared::Dispute),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "fee_refund"))]
    ApplicationFeeRefund(stripe_shared::ApplicationFeeRefund),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "issuing.authorization")
    )]
    IssuingAuthorization(stripe_shared::IssuingAuthorization),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "issuing.dispute")
    )]
    IssuingDispute(stripe_shared::IssuingDispute),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "issuing.transaction")
    )]
    IssuingTransaction(stripe_shared::IssuingTransaction),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "payout"))]
    Payout(stripe_shared::Payout),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "refund"))]
    Refund(stripe_shared::Refund),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "reserve_transaction")
    )]
    ReserveTransaction(stripe_shared::ReserveTransaction),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "tax_deducted_at_source")
    )]
    TaxDeductedAtSource(stripe_shared::TaxDeductedAtSource),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "topup"))]
    Topup(stripe_shared::Topup),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "transfer"))]
    Transfer(stripe_shared::Transfer),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "transfer_reversal")
    )]
    TransferReversal(stripe_shared::TransferReversal),
}

const _: () = {
    use stripe_miniserde::de::Visitor;
    use stripe_miniserde::{Deserialize, Error, Result, make_place};
    use stripe_miniserde::json::peek_object_tag;

    use super::*;

    make_place!(Place);

    impl Deserialize for BalanceTransactionSource {
        const WANTS_RAW: bool = true;

        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<BalanceTransactionSource> {
        fn wants_raw(&self) -> bool {
            true
        }

        fn raw(&mut self, bytes: &str) -> Result<()> {
            let tag = peek_object_tag(bytes).ok_or(Error)?;
            self.out = Some(match tag.as_str() {
                "application_fee" => {
                    BalanceTransactionSource::ApplicationFee(stripe_miniserde::json::from_str(bytes)?)
                }
                "charge" => BalanceTransactionSource::Charge(stripe_miniserde::json::from_str(bytes)?),
                "connect_collection_transfer" => {
                    BalanceTransactionSource::ConnectCollectionTransfer(stripe_miniserde::json::from_str(
                        bytes,
                    )?)
                }
                "customer_cash_balance_transaction" => {
                    BalanceTransactionSource::CustomerCashBalanceTransaction(
                        stripe_miniserde::json::from_str(bytes)?,
                    )
                }
                "dispute" => BalanceTransactionSource::Dispute(stripe_miniserde::json::from_str(bytes)?),
                "fee_refund" => BalanceTransactionSource::ApplicationFeeRefund(
                    stripe_miniserde::json::from_str(bytes)?,
                ),
                "issuing.authorization" => BalanceTransactionSource::IssuingAuthorization(
                    stripe_miniserde::json::from_str(bytes)?,
                ),
                "issuing.dispute" => {
                    BalanceTransactionSource::IssuingDispute(stripe_miniserde::json::from_str(bytes)?)
                }
                "issuing.transaction" => {
                    BalanceTransactionSource::IssuingTransaction(stripe_miniserde::json::from_str(bytes)?)
                }
                "payout" => BalanceTransactionSource::Payout(stripe_miniserde::json::from_str(bytes)?),
                "refund" => BalanceTransactionSource::Refund(stripe_miniserde::json::from_str(bytes)?),
                "reserve_transaction" => {
                    BalanceTransactionSource::ReserveTransaction(stripe_miniserde::json::from_str(bytes)?)
                }
                "tax_deducted_at_source" => {
                    BalanceTransactionSource::TaxDeductedAtSource(stripe_miniserde::json::from_str(bytes)?)
                }
                "topup" => BalanceTransactionSource::Topup(stripe_miniserde::json::from_str(bytes)?),
                "transfer" => BalanceTransactionSource::Transfer(stripe_miniserde::json::from_str(bytes)?),
                "transfer_reversal" => {
                    BalanceTransactionSource::TransferReversal(stripe_miniserde::json::from_str(bytes)?)
                }

                _ => return Err(Error),
            });
            Ok(())
        }
    }
};

#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceTransactionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceTransactionSource").finish_non_exhaustive()
    }
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
            Self::Refund(v) => v.id.inner(),
            Self::ReserveTransaction(v) => v.id.inner(),
            Self::TaxDeductedAtSource(v) => v.id.inner(),
            Self::Topup(v) => v.id.inner(),
            Self::Transfer(v) => v.id.inner(),
            Self::TransferReversal(v) => v.id.inner(),
        }
    }

    fn into_id(self) -> Self::Id {
        match self {
            Self::ApplicationFee(v) => v.id.into_inner(),
            Self::Charge(v) => v.id.into_inner(),
            Self::ConnectCollectionTransfer(v) => v.id.into_inner(),
            Self::CustomerCashBalanceTransaction(v) => v.id.into_inner(),
            Self::Dispute(v) => v.id.into_inner(),
            Self::ApplicationFeeRefund(v) => v.id.into_inner(),
            Self::IssuingAuthorization(v) => v.id.into_inner(),
            Self::IssuingDispute(v) => v.id.into_inner(),
            Self::IssuingTransaction(v) => v.id.into_inner(),
            Self::Payout(v) => v.id.into_inner(),
            Self::Refund(v) => v.id.into_inner(),
            Self::ReserveTransaction(v) => v.id.into_inner(),
            Self::TaxDeductedAtSource(v) => v.id.into_inner(),
            Self::Topup(v) => v.id.into_inner(),
            Self::Transfer(v) => v.id.into_inner(),
            Self::TransferReversal(v) => v.id.into_inner(),
        }
    }
}
