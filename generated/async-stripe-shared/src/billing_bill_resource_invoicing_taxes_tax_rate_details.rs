#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingTaxesTaxRateDetails {
    /// ID of the tax rate
    pub tax_rate: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxRateDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingTaxesTaxRateDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder {
    tax_rate: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingBillResourceInvoicingTaxesTaxRateDetailsBuilder {
                    tax_rate: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_rate" => Deserialize::begin(&mut self.builder.tax_rate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(tax_rate),) = (self.builder.tax_rate.take(),) else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingTaxesTaxRateDetails { tax_rate });
            Ok(())
        }
    }
};
