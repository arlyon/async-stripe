#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceSettingCustomerSetting {
    /// Default custom fields to be displayed on invoices for this customer.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// Default footer to be displayed on invoices for this customer.
    pub footer: Option<String>,
    /// Default options for invoice PDF rendering for this customer.
    pub rendering_options: Option<stripe_shared::InvoiceSettingCustomerRenderingOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceSettingCustomerSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceSettingCustomerSetting").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceSettingCustomerSettingBuilder {
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    footer: Option<Option<String>>,
    rendering_options: Option<Option<stripe_shared::InvoiceSettingCustomerRenderingOptions>>,
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

    impl Deserialize for InvoiceSettingCustomerSetting {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingCustomerSetting>,
        builder: InvoiceSettingCustomerSettingBuilder,
    }

    impl Visitor for Place<InvoiceSettingCustomerSetting> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceSettingCustomerSettingBuilder {
                    custom_fields: Deserialize::default(),
                    default_payment_method: Deserialize::default(),
                    footer: Deserialize::default(),
                    rendering_options: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_fields" => Deserialize::begin(&mut self.builder.custom_fields),
                "default_payment_method" => {
                    Deserialize::begin(&mut self.builder.default_payment_method)
                }
                "footer" => Deserialize::begin(&mut self.builder.footer),
                "rendering_options" => Deserialize::begin(&mut self.builder.rendering_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(custom_fields),
                Some(default_payment_method),
                Some(footer),
                Some(rendering_options),
            ) = (
                self.builder.custom_fields.take(),
                self.builder.default_payment_method.take(),
                self.builder.footer.take(),
                self.builder.rendering_options.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceSettingCustomerSetting {
                custom_fields,
                default_payment_method,
                footer,
                rendering_options,
            });
            Ok(())
        }
    }
};
