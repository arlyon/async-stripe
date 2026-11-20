#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax {
    /// Indicates whether the transaction is tax exempt.
pub tax_exempt_indicator: Option<bool>,
    /// Tax details.
pub tax_items: Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxItem>>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxBuilder {
    tax_exempt_indicator: Option<Option<bool>>,
tax_items: Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxItem>>>,

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

    impl Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax>,
        builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxBuilder,
    }

    impl Visitor for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxBuilder {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_exempt_indicator" => Deserialize::begin(&mut self.tax_exempt_indicator),
                "tax_items" => Deserialize::begin(&mut self.tax_items),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tax_exempt_indicator: Some(None), tax_items: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(tax_exempt_indicator), Some(tax_items)) =
                (self.tax_exempt_indicator, self.tax_items.take())
            else {
                return None;
            };
            Some(Self::Out { tax_exempt_indicator, tax_items })
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax {
        type Builder =
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "tax_exempt_indicator" => b.tax_exempt_indicator = FromValueOpt::from_value(v),
                    "tax_items" => b.tax_items = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
