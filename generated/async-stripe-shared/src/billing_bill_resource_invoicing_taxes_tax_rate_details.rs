#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingTaxesTaxRateDetails {
    pub tax_rate: String,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder {
    tax_rate: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BillingBillResourceInvoicingTaxesTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingTaxesTaxRateDetails>,
        builder: BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingTaxesTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder {
        type Out = BillingBillResourceInvoicingTaxesTaxRateDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_rate" => Deserialize::begin(&mut self.tax_rate),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tax_rate: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(tax_rate),) = (self.tax_rate.take(),) else {
                return None;
            };
            Some(Self::Out { tax_rate })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingBillResourceInvoicingTaxesTaxRateDetails {
        type Builder = BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingTaxesTaxRateDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "tax_rate" => b.tax_rate = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
