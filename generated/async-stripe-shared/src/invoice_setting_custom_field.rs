#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,
    /// The value of the custom field.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceSettingCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceSettingCustomField").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceSettingCustomFieldBuilder {
    name: Option<String>,
    value: Option<String>,
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

    impl Deserialize for InvoiceSettingCustomField {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceSettingCustomField>,
        builder: InvoiceSettingCustomFieldBuilder,
    }

    impl Visitor for Place<InvoiceSettingCustomField> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceSettingCustomFieldBuilder {
                    name: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "name" => Deserialize::begin(&mut self.builder.name),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(name), Some(value)) = (self.builder.name.take(), self.builder.value.take())
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceSettingCustomField { name, value });
            Ok(())
        }
    }
};
