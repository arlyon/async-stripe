/// Result from a email check
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoEmailReport {
    /// Email to be verified.
    pub email: Option<String>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoEmailReportError>,
    /// Status of this `email` check.
    pub status: GelatoEmailReportStatus,
}
#[doc(hidden)]
pub struct GelatoEmailReportBuilder {
    email: Option<Option<String>>,
    error: Option<Option<stripe_misc::GelatoEmailReportError>>,
    status: Option<GelatoEmailReportStatus>,
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

    impl Deserialize for GelatoEmailReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoEmailReport>,
        builder: GelatoEmailReportBuilder,
    }

    impl Visitor for Place<GelatoEmailReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoEmailReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoEmailReportBuilder {
        type Out = GelatoEmailReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "email" => Deserialize::begin(&mut self.email),
                "error" => Deserialize::begin(&mut self.error),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                email: Deserialize::default(),
                error: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(email), Some(error), Some(status)) =
                (self.email.take(), self.error.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { email, error, status })
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

    impl ObjectDeser for GelatoEmailReport {
        type Builder = GelatoEmailReportBuilder;
    }

    impl FromValueOpt for GelatoEmailReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoEmailReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "email" => b.email = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `email` check.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoEmailReportStatus {
    Unverified,
    Verified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoEmailReportStatus {
    pub fn as_str(&self) -> &str {
        use GelatoEmailReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoEmailReportStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoEmailReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoEmailReportStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoEmailReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoEmailReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoEmailReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoEmailReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoEmailReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoEmailReportStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoEmailReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoEmailReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
