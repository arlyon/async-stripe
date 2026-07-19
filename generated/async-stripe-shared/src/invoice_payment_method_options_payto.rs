#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsPayto {
    pub mandate_options: Option<stripe_shared::InvoiceMandateOptionsPayto>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsPaytoBuilder {
    mandate_options: Option<Option<stripe_shared::InvoiceMandateOptionsPayto>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsPayto>,
        builder: InvoicePaymentMethodOptionsPaytoBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsPaytoBuilder {
                    mandate_options: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mandate_options" => Deserialize::begin(&mut self.builder.mandate_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(mandate_options),) = (self.builder.mandate_options.take(),) else {
                return Ok(());
            };
            *self.out = Some(InvoicePaymentMethodOptionsPayto { mandate_options });
            Ok(())
        }
    }
};
