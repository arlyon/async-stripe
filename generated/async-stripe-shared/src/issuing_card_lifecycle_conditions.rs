#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardLifecycleConditions {
    /// The card is automatically cancelled when it makes this number of non-zero payment authorizations and transactions.
    /// The count includes penny authorizations, but doesn't include non-payment actions, such as authorization advice.
    pub payment_count: u64,
}
#[doc(hidden)]
pub struct IssuingCardLifecycleConditionsBuilder {
    payment_count: Option<u64>,
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

    impl Deserialize for IssuingCardLifecycleConditions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardLifecycleConditions>,
        builder: IssuingCardLifecycleConditionsBuilder,
    }

    impl Visitor for Place<IssuingCardLifecycleConditions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardLifecycleConditionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardLifecycleConditionsBuilder {
        type Out = IssuingCardLifecycleConditions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_count" => Deserialize::begin(&mut self.payment_count),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_count: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_count),) = (self.payment_count,) else {
                return None;
            };
            Some(Self::Out { payment_count })
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

    impl ObjectDeser for IssuingCardLifecycleConditions {
        type Builder = IssuingCardLifecycleConditionsBuilder;
    }

    impl FromValueOpt for IssuingCardLifecycleConditions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardLifecycleConditionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_count" => b.payment_count = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
