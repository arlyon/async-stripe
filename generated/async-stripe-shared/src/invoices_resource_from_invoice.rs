#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceFromInvoice {
    /// The relation between this invoice and the cloned invoice
    pub action: String,
    /// The invoice that was cloned.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
}
#[doc(hidden)]
pub struct InvoicesResourceFromInvoiceBuilder {
    action: Option<String>,
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
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

    impl Deserialize for InvoicesResourceFromInvoice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceFromInvoice>,
        builder: InvoicesResourceFromInvoiceBuilder,
    }

    impl Visitor for Place<InvoicesResourceFromInvoice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceFromInvoiceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceFromInvoiceBuilder {
        type Out = InvoicesResourceFromInvoice;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.action),
                "invoice" => Deserialize::begin(&mut self.invoice),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { action: Deserialize::default(), invoice: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(action), Some(invoice)) = (self.action.take(), self.invoice.take()) else {
                return None;
            };
            Some(Self::Out { action, invoice })
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

    impl ObjectDeser for InvoicesResourceFromInvoice {
        type Builder = InvoicesResourceFromInvoiceBuilder;
    }

    impl FromValueOpt for InvoicesResourceFromInvoice {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceFromInvoiceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "action" => b.action = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
