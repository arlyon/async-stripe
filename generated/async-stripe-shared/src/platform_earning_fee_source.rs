#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PlatformEarningFeeSource {
    /// Charge ID that created this application fee.
    pub charge: Option<String>,
    /// Payout ID that created this application fee.
    pub payout: Option<String>,
    /// Type of object that created the application fee.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PlatformEarningFeeSourceType,
}
#[doc(hidden)]
pub struct PlatformEarningFeeSourceBuilder {
    charge: Option<Option<String>>,
    payout: Option<Option<String>>,
    type_: Option<PlatformEarningFeeSourceType>,
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

    impl Deserialize for PlatformEarningFeeSource {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PlatformEarningFeeSource>,
        builder: PlatformEarningFeeSourceBuilder,
    }

    impl Visitor for Place<PlatformEarningFeeSource> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PlatformEarningFeeSourceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PlatformEarningFeeSourceBuilder {
        type Out = PlatformEarningFeeSource;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.charge),
                "payout" => Deserialize::begin(&mut self.payout),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                charge: Deserialize::default(),
                payout: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(charge), Some(payout), Some(type_)) =
                (self.charge.take(), self.payout.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { charge, payout, type_ })
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

    impl ObjectDeser for PlatformEarningFeeSource {
        type Builder = PlatformEarningFeeSourceBuilder;
    }

    impl FromValueOpt for PlatformEarningFeeSource {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PlatformEarningFeeSourceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "payout" => b.payout = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of object that created the application fee.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlatformEarningFeeSourceType {
    Charge,
    Payout,
}
impl PlatformEarningFeeSourceType {
    pub fn as_str(self) -> &'static str {
        use PlatformEarningFeeSourceType::*;
        match self {
            Charge => "charge",
            Payout => "payout",
        }
    }
}

impl std::str::FromStr for PlatformEarningFeeSourceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlatformEarningFeeSourceType::*;
        match s {
            "charge" => Ok(Charge),
            "payout" => Ok(Payout),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PlatformEarningFeeSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlatformEarningFeeSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PlatformEarningFeeSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PlatformEarningFeeSourceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlatformEarningFeeSourceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlatformEarningFeeSourceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PlatformEarningFeeSourceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlatformEarningFeeSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PlatformEarningFeeSourceType"))
    }
}
