#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdown {
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsOption>,
    /// The option selected by the customer. This will be the `value` for the option.
    pub value: Option<String>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder {
    options: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsOption>>,
    value: Option<Option<String>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsDropdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsDropdown>,
        builder: PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsDropdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFieldsDropdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "options" => Deserialize::begin(&mut self.options),
                "value" => Deserialize::begin(&mut self.value),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { options: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(options), Some(value)) = (self.options.take(), self.value.take()) else {
                return None;
            };
            Some(Self::Out { options, value })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFieldsDropdown {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomFieldsDropdown {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCustomFieldsDropdownBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "options" => b.options = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
