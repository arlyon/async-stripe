#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
    /// The value entered by the customer, containing only digits.
    pub value: Option<String>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomFieldsNumericBuilder {
    maximum_length: Option<Option<i64>>,
    minimum_length: Option<Option<i64>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsNumeric {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsNumeric>,
        builder: PaymentPagesCheckoutSessionCustomFieldsNumericBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsNumeric> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomFieldsNumericBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsNumericBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFieldsNumeric;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "maximum_length" => Deserialize::begin(&mut self.maximum_length),
                "minimum_length" => Deserialize::begin(&mut self.minimum_length),
                "value" => Deserialize::begin(&mut self.value),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                maximum_length: Deserialize::default(),
                minimum_length: Deserialize::default(),
                value: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(maximum_length), Some(minimum_length), Some(value)) =
                (self.maximum_length, self.minimum_length, self.value.take())
            else {
                return None;
            };
            Some(Self::Out { maximum_length, minimum_length, value })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFieldsNumeric {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsNumericBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomFieldsNumeric {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCustomFieldsNumericBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "maximum_length" => b.maximum_length = FromValueOpt::from_value(v),
                    "minimum_length" => b.minimum_length = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
