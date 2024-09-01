#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsDropdownOption {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsDropdownOptionBuilder {
    label: Option<String>,
    value: Option<String>,
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

    impl Deserialize for PaymentLinksResourceCustomFieldsDropdownOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsDropdownOption>,
        builder: PaymentLinksResourceCustomFieldsDropdownOptionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsDropdownOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsDropdownOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsDropdownOptionBuilder {
        type Out = PaymentLinksResourceCustomFieldsDropdownOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "label" => Deserialize::begin(&mut self.label),
                "value" => Deserialize::begin(&mut self.value),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { label: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(label), Some(value)) = (self.label.take(), self.value.take()) else {
                return None;
            };
            Some(Self::Out { label, value })
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

    impl ObjectDeser for PaymentLinksResourceCustomFieldsDropdownOption {
        type Builder = PaymentLinksResourceCustomFieldsDropdownOptionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCustomFieldsDropdownOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCustomFieldsDropdownOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "label" => b.label = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
