#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceFromInvoice {
    /// The relation between this invoice and the cloned invoice
    pub action: String,
    /// The invoice that was cloned.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesResourceFromInvoice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesResourceFromInvoice").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesResourceFromInvoiceBuilder {
    action: Option<String>,
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
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
                builder: InvoicesResourceFromInvoiceBuilder {
                    action: Deserialize::default(),
                    invoice: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.builder.action),
                "invoice" => Deserialize::begin(&mut self.builder.invoice),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(action), Some(invoice)) =
                (self.builder.action.take(), self.builder.invoice.take())
            else {
                return Ok(());
            };
            *self.out = Some(InvoicesResourceFromInvoice { action, invoice });
            Ok(())
        }
    }
};
