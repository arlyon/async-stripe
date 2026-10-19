#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,
    pub rate: stripe_shared::TaxRate,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<stripe_shared::TaxabilityReason>,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LineItemsTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LineItemsTaxAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct LineItemsTaxAmountBuilder {
    amount: Option<i64>,
    rate: Option<stripe_shared::TaxRate>,
    taxability_reason: Option<Option<stripe_shared::TaxabilityReason>>,
    taxable_amount: Option<Option<i64>>,
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

    impl Deserialize for LineItemsTaxAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LineItemsTaxAmount>,
        builder: LineItemsTaxAmountBuilder,
    }

    impl Visitor for Place<LineItemsTaxAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LineItemsTaxAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LineItemsTaxAmountBuilder {
        type Out = LineItemsTaxAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "rate" => Deserialize::begin(&mut self.rate),
                "taxability_reason" => Deserialize::begin(&mut self.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.taxable_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: None,
                rate: None,
                taxability_reason: Some(None),
                taxable_amount: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(rate), Some(taxability_reason), Some(taxable_amount)) =
                (self.amount, self.rate.take(), self.taxability_reason.take(), self.taxable_amount)
            else {
                return None;
            };
            Some(Self::Out { amount, rate, taxability_reason, taxable_amount })
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

    impl ObjectDeser for LineItemsTaxAmount {
        type Builder = LineItemsTaxAmountBuilder;
    }

    impl FromValueOpt for LineItemsTaxAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LineItemsTaxAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "rate" => b.rate = FromValueOpt::from_value(v),
                    "taxability_reason" => b.taxability_reason = FromValueOpt::from_value(v),
                    "taxable_amount" => b.taxable_amount = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// The possible values for this field may be extended as new tax rules are supported.
pub type LineItemsTaxAmountTaxabilityReason = stripe_shared::TaxabilityReason;
