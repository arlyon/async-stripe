#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsMultibanco {
    /// Entity number associated with this Multibanco payment.
    pub entity: Option<String>,
    /// Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsMultibancoBuilder {
    entity: Option<Option<String>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsMultibanco {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsMultibanco>,
        builder: PaymentMethodDetailsMultibancoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsMultibanco> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsMultibancoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsMultibancoBuilder {
        type Out = PaymentMethodDetailsMultibanco;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "entity" => Deserialize::begin(&mut self.entity),
                "reference" => Deserialize::begin(&mut self.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { entity: Deserialize::default(), reference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(entity), Some(reference)) = (self.entity.take(), self.reference.take())
            else {
                return None;
            };
            Some(Self::Out { entity, reference })
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

    impl ObjectDeser for PaymentMethodDetailsMultibanco {
        type Builder = PaymentMethodDetailsMultibancoBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsMultibanco {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsMultibancoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "entity" => b.entity = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
