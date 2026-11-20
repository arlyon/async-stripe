#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal {
    /// Total price of the lodging reservation in cents.
pub amount: Option<i64>,
    /// Cash advances in cents.
pub cash_advances: Option<i64>,
    /// Currency of the total amount.
pub currency: Option<stripe_types::Currency>,
pub discounts: Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount>,
    /// Additional charges for the lodging.
pub extra_charges: Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryExtraCharge>>,
    /// Prepaid amount in cents.
pub prepaid_amount: Option<i64>,
pub tax: Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotalBuilder {
    amount: Option<Option<i64>>,
cash_advances: Option<Option<i64>>,
currency: Option<Option<stripe_types::Currency>>,
discounts: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount>>,
extra_charges: Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryExtraCharge>>>,
prepaid_amount: Option<Option<i64>>,
tax: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax>>,

}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal,
        >,
        builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotalBuilder,
    }

    impl Visitor for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotalBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotalBuilder
    {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "cash_advances" => Deserialize::begin(&mut self.cash_advances),
                "currency" => Deserialize::begin(&mut self.currency),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "extra_charges" => Deserialize::begin(&mut self.extra_charges),
                "prepaid_amount" => Deserialize::begin(&mut self.prepaid_amount),
                "tax" => Deserialize::begin(&mut self.tax),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                cash_advances: Some(None),
                currency: Some(None),
                discounts: Some(None),
                extra_charges: Some(None),
                prepaid_amount: Some(None),
                tax: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(cash_advances),
                Some(currency),
                Some(discounts),
                Some(extra_charges),
                Some(prepaid_amount),
                Some(tax),
            ) = (
                self.amount,
                self.cash_advances,
                self.currency.take(),
                self.discounts.take(),
                self.extra_charges.take(),
                self.prepaid_amount,
                self.tax.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                cash_advances,
                currency,
                discounts,
                extra_charges,
                prepaid_amount,
                tax,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal {
        type Builder =
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotalBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTotalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "cash_advances" => b.cash_advances = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "extra_charges" => b.extra_charges = FromValueOpt::from_value(v),
                    "prepaid_amount" => b.prepaid_amount = FromValueOpt::from_value(v),
                    "tax" => b.tax = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
