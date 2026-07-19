#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Options for invoice PDF rendering.
    pub rendering_options: Option<stripe_shared::InvoiceSettingCheckoutRenderingOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceInvoiceSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceInvoiceSettingsBuilder {
    account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    description: Option<Option<String>>,
    footer: Option<Option<String>>,
    issuer: Option<Option<stripe_shared::ConnectAccountReference>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    rendering_options: Option<Option<stripe_shared::InvoiceSettingCheckoutRenderingOptions>>,
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
                builder: PaymentLinksResourceInvoiceSettingsBuilder {
                    account_tax_ids: Deserialize::default(),
                    custom_fields: Deserialize::default(),
                    description: Deserialize::default(),
                    footer: Deserialize::default(),
                    issuer: Deserialize::default(),
                    metadata: Deserialize::default(),
                    rendering_options: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_tax_ids" => Deserialize::begin(&mut self.builder.account_tax_ids),
                "custom_fields" => Deserialize::begin(&mut self.builder.custom_fields),
                "description" => Deserialize::begin(&mut self.builder.description),
                "footer" => Deserialize::begin(&mut self.builder.footer),
                "issuer" => Deserialize::begin(&mut self.builder.issuer),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "rendering_options" => Deserialize::begin(&mut self.builder.rendering_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_tax_ids),
                Some(custom_fields),
                Some(description),
                Some(footer),
                Some(issuer),
                Some(metadata),
                Some(rendering_options),
            ) = (
                self.builder.account_tax_ids.take(),
                self.builder.custom_fields.take(),
                self.builder.description.take(),
                self.builder.footer.take(),
                self.builder.issuer.take(),
                self.builder.metadata.take(),
                self.builder.rendering_options.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceInvoiceSettings {
                account_tax_ids,
                custom_fields,
                description,
                footer,
                issuer,
                metadata,
                rendering_options,
            });
            Ok(())
        }
    }
};
