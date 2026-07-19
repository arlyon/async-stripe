#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingLinesCommonCreditedItems {
    /// Invoice containing the credited invoice line items
    pub invoice: String,
    /// Credited invoice line items
    pub invoice_line_items: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingLinesCommonCreditedItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingLinesCommonCreditedItems")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingLinesCommonCreditedItemsBuilder {
    invoice: Option<String>,
    invoice_line_items: Option<Vec<String>>,
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

    impl Deserialize for BillingBillResourceInvoicingLinesCommonCreditedItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingLinesCommonCreditedItems>,
        builder: BillingBillResourceInvoicingLinesCommonCreditedItemsBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingLinesCommonCreditedItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingBillResourceInvoicingLinesCommonCreditedItemsBuilder {
                    invoice: Deserialize::default(),
                    invoice_line_items: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice" => Deserialize::begin(&mut self.builder.invoice),
                "invoice_line_items" => Deserialize::begin(&mut self.builder.invoice_line_items),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(invoice), Some(invoice_line_items)) =
                (self.builder.invoice.take(), self.builder.invoice_line_items.take())
            else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingLinesCommonCreditedItems {
                invoice,
                invoice_line_items,
            });
            Ok(())
        }
    }
};
