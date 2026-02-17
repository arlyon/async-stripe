#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceSettingCustomerRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,
    /// ID of the invoice rendering template to be used for this customer's invoices.
    /// If set, the template will be used on all invoices for this customer unless a template is set directly on the invoice.
    pub template: Option<String>,
}
#[doc(hidden)]
pub struct InvoiceSettingCustomerRenderingOptionsBuilder {
    amount_tax_display: Option<Option<String>>,
    template: Option<Option<String>>,
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
                builder: InvoiceSettingCustomerRenderingOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceSettingCustomerRenderingOptionsBuilder {
        type Out = InvoiceSettingCustomerRenderingOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_tax_display" => Deserialize::begin(&mut self.amount_tax_display),
                "template" => Deserialize::begin(&mut self.template),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount_tax_display: Deserialize::default(), template: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_tax_display), Some(template)) =
                (self.amount_tax_display.take(), self.template.take())
            else {
                return None;
            };
            Some(Self::Out { amount_tax_display, template })
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

    impl ObjectDeser for InvoiceSettingCustomerRenderingOptions {
        type Builder = InvoiceSettingCustomerRenderingOptionsBuilder;
    }

    impl FromValueOpt for InvoiceSettingCustomerRenderingOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceSettingCustomerRenderingOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_tax_display" => b.amount_tax_display = FromValueOpt::from_value(v),
                    "template" => b.template = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
