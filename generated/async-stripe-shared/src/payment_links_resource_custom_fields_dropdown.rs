#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<stripe_shared::PaymentLinksResourceCustomFieldsDropdownOption>,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsDropdownBuilder {
    options: Option<Vec<stripe_shared::PaymentLinksResourceCustomFieldsDropdownOption>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceCustomFieldsDropdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsDropdown>,
        builder: PaymentLinksResourceCustomFieldsDropdownBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsDropdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsDropdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsDropdownBuilder {
        type Out = PaymentLinksResourceCustomFieldsDropdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "options" => Deserialize::begin(&mut self.options),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { options: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { options: self.options.take()? })
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

    impl ObjectDeser for PaymentLinksResourceCustomFieldsDropdown {
        type Builder = PaymentLinksResourceCustomFieldsDropdownBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCustomFieldsDropdown {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCustomFieldsDropdownBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "options" => b.options = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
