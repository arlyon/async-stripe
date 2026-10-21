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
    invoice_data: Option<stripe_shared::PaymentLinksResourceInvoiceSettings>,
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
                builder: PaymentLinksResourceInvoiceCreationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceInvoiceCreationBuilder {
        type Out = PaymentLinksResourceInvoiceCreation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "invoice_data" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.invoice_data)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: None, invoice_data: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { enabled: Some(enabled), invoice_data } => {
                    Some(Self::Out { enabled, invoice_data })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentLinksResourceInvoiceCreation {
        type Builder = PaymentLinksResourceInvoiceCreationBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceInvoiceCreation {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
