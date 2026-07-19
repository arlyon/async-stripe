#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
    /// Invoice pdf rendering options
    pub pdf: Option<stripe_shared::InvoiceRenderingPdf>,
    /// ID of the rendering template that the invoice is formatted by.
    pub template: Option<String>,
    /// Version of the rendering template that the invoice is using.
    pub template_version: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesResourceInvoiceRendering {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesResourceInvoiceRendering").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesResourceInvoiceRenderingBuilder {
    amount_tax_display: Option<Option<String>>,
    pdf: Option<Option<stripe_shared::InvoiceRenderingPdf>>,
    template: Option<Option<String>>,
    template_version: Option<Option<i64>>,
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

    impl Deserialize for InvoicesResourceInvoiceRendering {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceInvoiceRendering>,
        builder: InvoicesResourceInvoiceRenderingBuilder,
    }

    impl Visitor for Place<InvoicesResourceInvoiceRendering> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceInvoiceRenderingBuilder {
                    amount_tax_display: Deserialize::default(),
                    pdf: Deserialize::default(),
                    template: Deserialize::default(),
                    template_version: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_tax_display" => Deserialize::begin(&mut self.builder.amount_tax_display),
                "pdf" => Deserialize::begin(&mut self.builder.pdf),
                "template" => Deserialize::begin(&mut self.builder.template),
                "template_version" => Deserialize::begin(&mut self.builder.template_version),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_tax_display), Some(pdf), Some(template), Some(template_version)) = (
                self.builder.amount_tax_display.take(),
                self.builder.pdf.take(),
                self.builder.template.take(),
                self.builder.template_version,
            ) else {
                return Ok(());
            };
            *self.out = Some(InvoicesResourceInvoiceRendering {
                amount_tax_display,
                pdf,
                template,
                template_version,
            });
            Ok(())
        }
    }
};
