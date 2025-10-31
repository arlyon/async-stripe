#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsText {
    /// The value that will pre-fill the field on the payment page.
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsTextBuilder {
    default_value: Option<Option<String>>,
    maximum_length: Option<Option<i64>>,
    minimum_length: Option<Option<i64>>,
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

    impl Deserialize for PaymentLinksResourceCustomFieldsText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsText>,
        builder: PaymentLinksResourceCustomFieldsTextBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsTextBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsTextBuilder {
        type Out = PaymentLinksResourceCustomFieldsText;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_value" => Deserialize::begin(&mut self.default_value),
                "maximum_length" => Deserialize::begin(&mut self.maximum_length),
                "minimum_length" => Deserialize::begin(&mut self.minimum_length),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                default_value: Deserialize::default(),
                maximum_length: Deserialize::default(),
                minimum_length: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(default_value), Some(maximum_length), Some(minimum_length)) =
                (self.default_value.take(), self.maximum_length, self.minimum_length)
            else {
                return None;
            };
            Some(Self::Out { default_value, maximum_length, minimum_length })
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

    impl ObjectDeser for PaymentLinksResourceCustomFieldsText {
        type Builder = PaymentLinksResourceCustomFieldsTextBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCustomFieldsText {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCustomFieldsTextBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "default_value" => b.default_value = FromValueOpt::from_value(v),
                    "maximum_length" => b.maximum_length = FromValueOpt::from_value(v),
                    "minimum_length" => b.minimum_length = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
