#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceInvoiceSettings {
    /// The account tax IDs associated with the invoice.
    pub account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// A list of up to 4 custom fields to be displayed on the invoice.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Footer to be displayed on the invoice.
    pub footer: Option<String>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    pub issuer: Option<stripe_shared::ConnectAccountReference>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Options for invoice PDF rendering.
    pub rendering_options: Option<stripe_shared::InvoiceSettingRenderingOptions>,
}
#[doc(hidden)]
pub struct PaymentLinksResourceInvoiceSettingsBuilder {
    account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    description: Option<Option<String>>,
    footer: Option<Option<String>>,
    issuer: Option<Option<stripe_shared::ConnectAccountReference>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    rendering_options: Option<Option<stripe_shared::InvoiceSettingRenderingOptions>>,
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

    impl Deserialize for PaymentLinksResourceInvoiceSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceInvoiceSettings>,
        builder: PaymentLinksResourceInvoiceSettingsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceInvoiceSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceInvoiceSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceInvoiceSettingsBuilder {
        type Out = PaymentLinksResourceInvoiceSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_tax_ids" => Deserialize::begin(&mut self.account_tax_ids),
                "custom_fields" => Deserialize::begin(&mut self.custom_fields),
                "description" => Deserialize::begin(&mut self.description),
                "footer" => Deserialize::begin(&mut self.footer),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "rendering_options" => Deserialize::begin(&mut self.rendering_options),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_tax_ids: Deserialize::default(),
                custom_fields: Deserialize::default(),
                description: Deserialize::default(),
                footer: Deserialize::default(),
                issuer: Deserialize::default(),
                metadata: Deserialize::default(),
                rendering_options: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_tax_ids),
                Some(custom_fields),
                Some(description),
                Some(footer),
                Some(issuer),
                Some(metadata),
                Some(rendering_options),
            ) = (
                self.account_tax_ids.take(),
                self.custom_fields.take(),
                self.description.take(),
                self.footer.take(),
                self.issuer.take(),
                self.metadata.take(),
                self.rendering_options.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_tax_ids,
                custom_fields,
                description,
                footer,
                issuer,
                metadata,
                rendering_options,
            })
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

    impl ObjectDeser for PaymentLinksResourceInvoiceSettings {
        type Builder = PaymentLinksResourceInvoiceSettingsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceInvoiceSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceInvoiceSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_tax_ids" => b.account_tax_ids = FromValueOpt::from_value(v),
                    "custom_fields" => b.custom_fields = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "footer" => b.footer = FromValueOpt::from_value(v),
                    "issuer" => b.issuer = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "rendering_options" => b.rendering_options = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
