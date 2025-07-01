#[derive(Clone, Debug)]
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
                builder: InvoicesResourceInvoiceRenderingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceInvoiceRenderingBuilder {
        type Out = InvoicesResourceInvoiceRendering;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_tax_display" => Deserialize::begin(&mut self.amount_tax_display),
                "pdf" => Deserialize::begin(&mut self.pdf),
                "template" => Deserialize::begin(&mut self.template),
                "template_version" => Deserialize::begin(&mut self.template_version),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_tax_display: Deserialize::default(),
                pdf: Deserialize::default(),
                template: Deserialize::default(),
                template_version: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_tax_display), Some(pdf), Some(template), Some(template_version)) = (
                self.amount_tax_display.take(),
                self.pdf,
                self.template.take(),
                self.template_version,
            ) else {
                return None;
            };
            Some(Self::Out { amount_tax_display, pdf, template, template_version })
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

    impl ObjectDeser for InvoicesResourceInvoiceRendering {
        type Builder = InvoicesResourceInvoiceRenderingBuilder;
    }

    impl FromValueOpt for InvoicesResourceInvoiceRendering {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceInvoiceRenderingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_tax_display" => b.amount_tax_display = FromValueOpt::from_value(v),
                    "pdf" => b.pdf = FromValueOpt::from_value(v),
                    "template" => b.template = FromValueOpt::from_value(v),
                    "template_version" => b.template_version = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
