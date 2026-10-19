#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceLineItemTaxRateDetails {
    /// A localized display name for tax type, intended to be human-readable.
    /// For example, "Local Sales and Use Tax", "Value-added tax (VAT)", or "Umsatzsteuer (USt.)".
    pub display_name: String,
    /// The tax rate percentage as a string. For example, 8.5% is represented as "8.5".
    pub percentage_decimal: String,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: stripe_shared::TaxType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceLineItemTaxRateDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceLineItemTaxRateDetailsBuilder {
    display_name: Option<String>,
    percentage_decimal: Option<String>,
    tax_type: Option<stripe_shared::TaxType>,
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

    impl Deserialize for TaxProductResourceLineItemTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceLineItemTaxRateDetails>,
        builder: TaxProductResourceLineItemTaxRateDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceLineItemTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceLineItemTaxRateDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceLineItemTaxRateDetailsBuilder {
        type Out = TaxProductResourceLineItemTaxRateDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.display_name),
                "percentage_decimal" => Deserialize::begin(&mut self.percentage_decimal),
                "tax_type" => Deserialize::begin(&mut self.tax_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { display_name: None, percentage_decimal: None, tax_type: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(display_name), Some(percentage_decimal), Some(tax_type)) =
                (self.display_name.take(), self.percentage_decimal.take(), self.tax_type.take())
            else {
                return None;
            };
            Some(Self::Out { display_name, percentage_decimal, tax_type })
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

    impl ObjectDeser for TaxProductResourceLineItemTaxRateDetails {
        type Builder = TaxProductResourceLineItemTaxRateDetailsBuilder;
    }

    impl FromValueOpt for TaxProductResourceLineItemTaxRateDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceLineItemTaxRateDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "percentage_decimal" => b.percentage_decimal = FromValueOpt::from_value(v),
                    "tax_type" => b.tax_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The tax type, such as `vat` or `sales_tax`.
pub type TaxProductResourceLineItemTaxRateDetailsTaxType = stripe_shared::TaxType;
