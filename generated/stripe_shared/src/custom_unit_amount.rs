#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomUnitAmount {
    /// The maximum unit amount the customer can specify for this item.
    pub maximum: Option<i64>,
    /// The minimum unit amount the customer can specify for this item.
    /// Must be at least the minimum charge amount.
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    pub preset: Option<i64>,
}
#[doc(hidden)]
pub struct CustomUnitAmountBuilder {
    maximum: Option<Option<i64>>,
    minimum: Option<Option<i64>>,
    preset: Option<Option<i64>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomUnitAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomUnitAmount>,
        builder: CustomUnitAmountBuilder,
    }

    impl Visitor for Place<CustomUnitAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomUnitAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomUnitAmountBuilder {
        type Out = CustomUnitAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "maximum" => Deserialize::begin(&mut self.maximum),
                "minimum" => Deserialize::begin(&mut self.minimum),
                "preset" => Deserialize::begin(&mut self.preset),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                maximum: Deserialize::default(),
                minimum: Deserialize::default(),
                preset: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { maximum: self.maximum?, minimum: self.minimum?, preset: self.preset? })
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

    impl ObjectDeser for CustomUnitAmount {
        type Builder = CustomUnitAmountBuilder;
    }

    impl FromValueOpt for CustomUnitAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomUnitAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "maximum" => b.maximum = Some(FromValueOpt::from_value(v)?),
                    "minimum" => b.minimum = Some(FromValueOpt::from_value(v)?),
                    "preset" => b.preset = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
