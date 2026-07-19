#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlatformEarningFeeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PlatformEarningFeeSource").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PlatformEarningFeeSourceBuilder {
                    charge: Deserialize::default(),
                    payout: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.builder.charge),
                "payout" => Deserialize::begin(&mut self.builder.payout),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(charge), Some(payout), Some(type_)) =
                (self.builder.charge.take(), self.builder.payout.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(PlatformEarningFeeSource { charge, payout, type_ });
            Ok(())
        }
    }
};
/// Type of object that created the application fee.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlatformEarningFeeSourceType {
    Charge,
    Payout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PlatformEarningFeeSourceType {
    pub fn as_str(&self) -> &str {
        use PlatformEarningFeeSourceType::*;
        match self {
            Charge => "charge",
            Payout => "payout",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PlatformEarningFeeSourceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlatformEarningFeeSourceType::*;
        match s {
            "charge" => Ok(Charge),
            "payout" => Ok(Payout),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PlatformEarningFeeSourceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PlatformEarningFeeSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PlatformEarningFeeSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlatformEarningFeeSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PlatformEarningFeeSourceType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PlatformEarningFeeSourceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PlatformEarningFeeSourceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlatformEarningFeeSourceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlatformEarningFeeSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
