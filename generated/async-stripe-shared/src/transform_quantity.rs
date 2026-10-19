#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: stripe_shared::Round,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransformQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransformQuantity").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TransformQuantityBuilder {
    divide_by: Option<i64>,
    round: Option<stripe_shared::Round>,
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

    impl Deserialize for TransformQuantity {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransformQuantity>,
        builder: TransformQuantityBuilder,
    }

    impl Visitor for Place<TransformQuantity> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TransformQuantityBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TransformQuantityBuilder {
        type Out = TransformQuantity;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "divide_by" => Deserialize::begin(&mut self.divide_by),
                "round" => Deserialize::begin(&mut self.round),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { divide_by: None, round: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(divide_by), Some(round)) = (self.divide_by, self.round.take()) else {
                return None;
            };
            Some(Self::Out { divide_by, round })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TransformQuantity {
        type Builder = TransformQuantityBuilder;
    }

    impl FromValueOpt for TransformQuantity {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TransformQuantityBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "divide_by" => b.divide_by = FromValueOpt::from_value(v),
                    "round" => b.round = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// After division, either round the result `up` or `down`.
pub type TransformQuantityRound = stripe_shared::Round;
