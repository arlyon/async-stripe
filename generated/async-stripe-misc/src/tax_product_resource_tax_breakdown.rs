#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub amount: i64,
    /// Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,
    pub tax_rate_details: stripe_misc::TaxProductResourceTaxRateDetails,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// We might extend the possible values for this field to support new tax rules.
    pub taxability_reason: stripe_shared::TaxabilityReason,
    /// The amount on which tax is calculated, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub taxable_amount: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxBreakdown").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxBreakdownBuilder {
    amount: Option<i64>,
    inclusive: Option<bool>,
    tax_rate_details: Option<stripe_misc::TaxProductResourceTaxRateDetails>,
    taxability_reason: Option<stripe_shared::TaxabilityReason>,
    taxable_amount: Option<i64>,
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

    impl Deserialize for TaxProductResourceTaxBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxBreakdown>,
        builder: TaxProductResourceTaxBreakdownBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxBreakdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxBreakdownBuilder {
        type Out = TaxProductResourceTaxBreakdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "inclusive" => Deserialize::begin(&mut self.inclusive),
                "tax_rate_details" => Deserialize::begin(&mut self.tax_rate_details),
                "taxability_reason" => Deserialize::begin(&mut self.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.taxable_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: None,
                inclusive: None,
                tax_rate_details: None,
                taxability_reason: None,
                taxable_amount: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(inclusive),
                Some(tax_rate_details),
                Some(taxability_reason),
                Some(taxable_amount),
            ) = (
                self.amount,
                self.inclusive,
                self.tax_rate_details.take(),
                self.taxability_reason.take(),
                self.taxable_amount,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                inclusive,
                tax_rate_details,
                taxability_reason,
                taxable_amount,
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

    impl ObjectDeser for TaxProductResourceTaxBreakdown {
        type Builder = TaxProductResourceTaxBreakdownBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxBreakdown {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxBreakdownBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "inclusive" => b.inclusive = FromValueOpt::from_value(v),
                    "tax_rate_details" => b.tax_rate_details = FromValueOpt::from_value(v),
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
/// We might extend the possible values for this field to support new tax rules.
pub type TaxProductResourceTaxBreakdownTaxabilityReason = stripe_shared::TaxabilityReason;
