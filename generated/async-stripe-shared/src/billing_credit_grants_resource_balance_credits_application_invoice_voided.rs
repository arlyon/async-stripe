#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided {
    /// The invoice to which the reinstated billing credits were originally applied.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
    /// The invoice line item to which the reinstated billing credits were originally applied.
    pub invoice_line_item: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoidedBuilder {
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    invoice_line_item: Option<String>,
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

    impl Deserialize for BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided>,
        builder: BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoidedBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoidedBuilder {
                    invoice: Deserialize::default(),
                    invoice_line_item: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice" => Deserialize::begin(&mut self.builder.invoice),
                "invoice_line_item" => Deserialize::begin(&mut self.builder.invoice_line_item),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(invoice), Some(invoice_line_item)) =
                (self.builder.invoice.take(), self.builder.invoice_line_item.take())
            else {
                return Ok(());
            };
            *self.out = Some(BillingCreditGrantsResourceBalanceCreditsApplicationInvoiceVoided {
                invoice,
                invoice_line_item,
            });
            Ok(())
        }
    }
};
