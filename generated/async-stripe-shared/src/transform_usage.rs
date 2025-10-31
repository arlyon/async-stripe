#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TransformUsage {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: TransformUsageRound,
}
#[doc(hidden)]
pub struct TransformUsageBuilder {
    divide_by: Option<i64>,
    round: Option<TransformUsageRound>,
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

    impl Deserialize for TransformUsage {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransformUsage>,
        builder: TransformUsageBuilder,
    }

    impl Visitor for Place<TransformUsage> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TransformUsageBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TransformUsageBuilder {
        type Out = TransformUsage;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "divide_by" => Deserialize::begin(&mut self.divide_by),
                "round" => Deserialize::begin(&mut self.round),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { divide_by: Deserialize::default(), round: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(divide_by), Some(round)) = (self.divide_by, self.round) else {
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TransformUsage {
        type Builder = TransformUsageBuilder;
    }

    impl FromValueOpt for TransformUsage {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TransformUsageBuilder::deser_default();
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransformUsageRound {
    Down,
    Up,
}
impl TransformUsageRound {
    pub fn as_str(self) -> &'static str {
        use TransformUsageRound::*;
        match self {
            Down => "down",
            Up => "up",
        }
    }
}

impl std::str::FromStr for TransformUsageRound {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransformUsageRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TransformUsageRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TransformUsageRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TransformUsageRound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TransformUsageRound {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TransformUsageRound> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransformUsageRound::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TransformUsageRound);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TransformUsageRound {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransformUsageRound"))
    }
}
