/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug)]
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
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "platform_tax_fee")
    )]
    PlatformTaxFee(stripe_shared::PlatformTaxFee),
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

#[derive(Default)]
pub struct BalanceTransactionSourceBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<BalanceTransactionSource>,
        builder: BalanceTransactionSourceBuilder,
    }

    impl Deserialize for BalanceTransactionSource {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<BalanceTransactionSource> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for BalanceTransactionSourceBuilder {
        type Out = BalanceTransactionSource;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            BalanceTransactionSource::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for BalanceTransactionSource {
        type Builder = BalanceTransactionSourceBuilder;
    }
    impl BalanceTransactionSource {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "application_fee" => {
                    Self::ApplicationFee(FromValueOpt::from_value(Value::Object(o))?)
                }
                "charge" => Self::Charge(FromValueOpt::from_value(Value::Object(o))?),
                "connect_collection_transfer" => {
                    Self::ConnectCollectionTransfer(FromValueOpt::from_value(Value::Object(o))?)
                }
                "customer_cash_balance_transaction" => Self::CustomerCashBalanceTransaction(
                    FromValueOpt::from_value(Value::Object(o))?,
                ),
                "dispute" => Self::Dispute(FromValueOpt::from_value(Value::Object(o))?),
                "fee_refund" => {
                    Self::ApplicationFeeRefund(FromValueOpt::from_value(Value::Object(o))?)
                }
                "issuing.authorization" => {
                    Self::IssuingAuthorization(FromValueOpt::from_value(Value::Object(o))?)
                }
                "issuing.dispute" => {
                    Self::IssuingDispute(FromValueOpt::from_value(Value::Object(o))?)
                }
                "issuing.transaction" => {
                    Self::IssuingTransaction(FromValueOpt::from_value(Value::Object(o))?)
                }
                "payout" => Self::Payout(FromValueOpt::from_value(Value::Object(o))?),
                "platform_tax_fee" => {
                    Self::PlatformTaxFee(FromValueOpt::from_value(Value::Object(o))?)
                }
                "refund" => Self::Refund(FromValueOpt::from_value(Value::Object(o))?),
                "reserve_transaction" => {
                    Self::ReserveTransaction(FromValueOpt::from_value(Value::Object(o))?)
                }
                "tax_deducted_at_source" => {
                    Self::TaxDeductedAtSource(FromValueOpt::from_value(Value::Object(o))?)
                }
                "topup" => Self::Topup(FromValueOpt::from_value(Value::Object(o))?),
                "transfer" => Self::Transfer(FromValueOpt::from_value(Value::Object(o))?),
                "transfer_reversal" => {
                    Self::TransferReversal(FromValueOpt::from_value(Value::Object(o))?)
                }

                _ => return None,
            })
        }
    }

    impl FromValueOpt for BalanceTransactionSource {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

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
            Self::PlatformTaxFee(v) => v.id.into_inner(),
            Self::Refund(v) => v.id.into_inner(),
            Self::ReserveTransaction(v) => v.id.into_inner(),
            Self::TaxDeductedAtSource(v) => v.id.into_inner(),
            Self::Topup(v) => v.id.into_inner(),
            Self::Transfer(v) => v.id.into_inner(),
            Self::TransferReversal(v) => v.id.into_inner(),
        }
    }
}
