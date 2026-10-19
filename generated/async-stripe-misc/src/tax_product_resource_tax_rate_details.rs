#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxRateDetails {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// The amount of the tax rate when the `rate_type` is `flat_amount`.
    /// Tax rates with `rate_type` `percentage` can vary based on the transaction, resulting in this field being `null`.
    /// This field exposes the amount and currency of the flat tax rate.
    pub flat_amount: Option<stripe_shared::TaxRateFlatAmount>,
    /// The tax rate percentage as a string. For example, 8.5% is represented as `"8.5"`.
    pub percentage_decimal: String,
    /// Indicates the type of tax rate applied to the taxable amount.
    /// This value can be `null` when no tax applies to the location.
    /// This field is only present for TaxRates created by Stripe Tax.
    pub rate_type: Option<stripe_shared::RateType>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: Option<String>,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<stripe_shared::TaxType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxRateDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxRateDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxRateDetailsBuilder {
    country: Option<Option<String>>,
    flat_amount: Option<Option<stripe_shared::TaxRateFlatAmount>>,
    percentage_decimal: Option<String>,
    rate_type: Option<Option<stripe_shared::RateType>>,
    state: Option<Option<String>>,
    tax_type: Option<Option<stripe_shared::TaxType>>,
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

    impl Deserialize for TaxProductResourceTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxRateDetails>,
        builder: TaxProductResourceTaxRateDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxRateDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxRateDetailsBuilder {
        type Out = TaxProductResourceTaxRateDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                "flat_amount" => Deserialize::begin(&mut self.flat_amount),
                "percentage_decimal" => Deserialize::begin(&mut self.percentage_decimal),
                "rate_type" => Deserialize::begin(&mut self.rate_type),
                "state" => Deserialize::begin(&mut self.state),
                "tax_type" => Deserialize::begin(&mut self.tax_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                country: Some(None),
                flat_amount: Some(None),
                percentage_decimal: None,
                rate_type: Some(None),
                state: Some(None),
                tax_type: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(country),
                Some(flat_amount),
                Some(percentage_decimal),
                Some(rate_type),
                Some(state),
                Some(tax_type),
            ) = (
                self.country.take(),
                self.flat_amount.take(),
                self.percentage_decimal.take(),
                self.rate_type.take(),
                self.state.take(),
                self.tax_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { country, flat_amount, percentage_decimal, rate_type, state, tax_type })
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

    impl ObjectDeser for TaxProductResourceTaxRateDetails {
        type Builder = TaxProductResourceTaxRateDetailsBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxRateDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxRateDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    "flat_amount" => b.flat_amount = FromValueOpt::from_value(v),
                    "percentage_decimal" => b.percentage_decimal = FromValueOpt::from_value(v),
                    "rate_type" => b.rate_type = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    "tax_type" => b.tax_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates the type of tax rate applied to the taxable amount.
/// This value can be `null` when no tax applies to the location.
/// This field is only present for TaxRates created by Stripe Tax.
pub type TaxProductResourceTaxRateDetailsRateType = stripe_shared::RateType;
/// The tax type, such as `vat` or `sales_tax`.
pub type TaxProductResourceTaxRateDetailsTaxType = stripe_shared::TaxType;
