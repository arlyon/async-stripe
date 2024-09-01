#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceLineItemsCreditedItems {
    /// Invoice containing the credited invoice line items
    pub invoice: String,
    /// Credited invoice line items
    pub invoice_line_items: Vec<String>,
}
#[doc(hidden)]
pub struct InvoicesResourceLineItemsCreditedItemsBuilder {
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesResourceLineItemsCreditedItems {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceLineItemsCreditedItems>,
        builder: InvoicesResourceLineItemsCreditedItemsBuilder,
    }

    impl Visitor for Place<InvoicesResourceLineItemsCreditedItems> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceLineItemsCreditedItemsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceLineItemsCreditedItemsBuilder {
        type Out = InvoicesResourceLineItemsCreditedItems;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_line_items" => Deserialize::begin(&mut self.invoice_line_items),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { invoice: Deserialize::default(), invoice_line_items: Deserialize::default() }
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

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for InvoicesResourceLineItemsCreditedItems {
        type Builder = InvoicesResourceLineItemsCreditedItemsBuilder;
    }

    impl FromValueOpt for InvoicesResourceLineItemsCreditedItems {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceLineItemsCreditedItemsBuilder::deser_default();
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
