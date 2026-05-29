#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditedItemsInvoiceLineItems {
    /// The invoice id for the debited line item(s).
    pub invoice: String,
    /// IDs of the debited invoice line item(s) on the invoice that correspond to the credit proration.
    pub invoice_line_items: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditedItemsInvoiceLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditedItemsInvoiceLineItems").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CreditedItemsInvoiceLineItemsBuilder {
    invoice: Option<String>,
    invoice_line_items: Option<Vec<String>>,
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

    impl Deserialize for CreditedItemsInvoiceLineItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditedItemsInvoiceLineItems>,
        builder: CreditedItemsInvoiceLineItemsBuilder,
    }

    impl Visitor for Place<CreditedItemsInvoiceLineItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditedItemsInvoiceLineItemsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditedItemsInvoiceLineItemsBuilder {
        type Out = CreditedItemsInvoiceLineItems;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_line_items" => Deserialize::begin(&mut self.invoice_line_items),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { invoice: None, invoice_line_items: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(invoice), Some(invoice_line_items)) =
                (self.invoice.take(), self.invoice_line_items.take())
            else {
                return None;
            };
            Some(Self::Out { invoice, invoice_line_items })
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

    impl ObjectDeser for CreditedItemsInvoiceLineItems {
        type Builder = CreditedItemsInvoiceLineItemsBuilder;
    }

    impl FromValueOpt for CreditedItemsInvoiceLineItems {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditedItemsInvoiceLineItemsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "invoice_line_items" => b.invoice_line_items = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
