#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceSettingCustomerRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
    /// ID of the invoice rendering template to be used for this customer's invoices.
    /// If set, the template will be used on all invoices for this customer unless a template is set directly on the invoice.
    pub template: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceSettingCustomerRenderingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceSettingCustomerRenderingOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceSettingCustomerRenderingOptionsBuilder {
    amount_tax_display: Option<Option<String>>,
    template: Option<Option<String>>,
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

    impl Deserialize for InvoiceSettingCustomerRenderingOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingCustomerRenderingOptions>,
        builder: InvoiceSettingCustomerRenderingOptionsBuilder,
    }

    impl Visitor for Place<InvoiceSettingCustomerRenderingOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceSettingCustomerRenderingOptionsBuilder {
                    amount_tax_display: Deserialize::default(),
                    template: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_tax_display" => Deserialize::begin(&mut self.builder.amount_tax_display),
                "template" => Deserialize::begin(&mut self.builder.template),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_tax_display), Some(template)) =
                (self.builder.amount_tax_display.take(), self.builder.template.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(InvoiceSettingCustomerRenderingOptions { amount_tax_display, template });
            Ok(())
        }
    }
};
