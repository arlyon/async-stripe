#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceInvoiceCreation {
    /// Enable creating an invoice on successful payment.
    pub enabled: bool,
    /// Configuration for the invoice. Default invoice values will be used if unspecified.
    pub invoice_data: Option<stripe_shared::PaymentLinksResourceInvoiceSettings>,
}
#[doc(hidden)]
pub struct PaymentLinksResourceInvoiceCreationBuilder {
    enabled: Option<bool>,
    invoice_data: Option<Option<stripe_shared::PaymentLinksResourceInvoiceSettings>>,
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
                "invoice_data" => Deserialize::begin(&mut self.invoice_data),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), invoice_data: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(invoice_data)) = (self.enabled, self.invoice_data.take())
            else {
                return None;
            };
            Some(Self::Out { enabled, invoice_data })
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

    impl ObjectDeser for PaymentLinksResourceInvoiceCreation {
        type Builder = PaymentLinksResourceInvoiceCreationBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceInvoiceCreation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceInvoiceCreationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "invoice_data" => b.invoice_data = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
