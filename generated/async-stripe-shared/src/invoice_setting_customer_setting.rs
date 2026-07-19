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
                builder: InvoiceSettingCustomerSettingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceSettingCustomerSettingBuilder {
        type Out = InvoiceSettingCustomerSetting;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_fields" => Deserialize::begin(&mut self.custom_fields),
                "default_payment_method" => Deserialize::begin(&mut self.default_payment_method),
                "footer" => Deserialize::begin(&mut self.footer),
                "rendering_options" => Deserialize::begin(&mut self.rendering_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom_fields: Some(None),
                default_payment_method: Some(None),
                footer: Some(None),
                rendering_options: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                custom_fields: self.custom_fields.take().flatten(),
                default_payment_method: self.default_payment_method.take().flatten(),
                footer: self.footer.take().flatten(),
                rendering_options: self.rendering_options.take().flatten(),
            })
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

    impl ObjectDeser for InvoiceSettingCustomerSetting {
        type Builder = InvoiceSettingCustomerSettingBuilder;
    }

    impl FromValueOpt for InvoiceSettingCustomerSetting {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceSettingCustomerSettingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom_fields" => b.custom_fields = FromValueOpt::from_value(v),
                    "default_payment_method" => {
                        b.default_payment_method = FromValueOpt::from_value(v)
                    }
                    "footer" => b.footer = FromValueOpt::from_value(v),
                    "rendering_options" => b.rendering_options = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
