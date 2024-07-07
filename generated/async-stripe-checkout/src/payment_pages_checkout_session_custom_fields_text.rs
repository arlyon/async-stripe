#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
    /// The value entered by the customer.
    pub value: Option<String>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomFieldsTextBuilder {
    maximum_length: Option<Option<i64>>,
    minimum_length: Option<Option<i64>>,
    value: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsText>,
        builder: PaymentPagesCheckoutSessionCustomFieldsTextBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomFieldsTextBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsTextBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFieldsText;
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
            Some(Self::Out {
                maximum_length: self.maximum_length?,
                minimum_length: self.minimum_length?,
                value: self.value.take()?,
            })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFieldsText {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsTextBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomFieldsText {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCustomFieldsTextBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "maximum_length" => b.maximum_length = Some(FromValueOpt::from_value(v)?),
                    "minimum_length" => b.minimum_length = Some(FromValueOpt::from_value(v)?),
                    "value" => b.value = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
