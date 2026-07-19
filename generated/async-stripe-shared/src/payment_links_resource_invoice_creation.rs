#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceInvoiceCreation {
    /// Enable creating an invoice on successful payment.
    pub enabled: bool,
    /// Configuration for the invoice. Default invoice values will be used if unspecified.
    pub invoice_data: Option<stripe_shared::PaymentLinksResourceInvoiceSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceInvoiceCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceInvoiceCreation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceInvoiceCreationBuilder {
    enabled: Option<bool>,
    invoice_data: Option<Option<stripe_shared::PaymentLinksResourceInvoiceSettings>>,
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

    impl Deserialize for PaymentLinksResourceInvoiceCreation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceInvoiceCreation>,
        builder: PaymentLinksResourceInvoiceCreationBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceInvoiceCreation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceInvoiceCreationBuilder {
                    enabled: Deserialize::default(),
                    invoice_data: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "invoice_data" => Deserialize::begin(&mut self.builder.invoice_data),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(invoice_data)) =
                (self.builder.enabled, self.builder.invoice_data.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceInvoiceCreation { enabled, invoice_data });
            Ok(())
        }
    }
};
