/// Result from a phone check
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoPhoneReport {
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoPhoneReportError>,
    /// Phone to be verified.
    pub phone: Option<String>,
    /// Status of this `phone` check.
    pub status: GelatoPhoneReportStatus,
}
#[doc(hidden)]
pub struct GelatoPhoneReportBuilder {
    error: Option<Option<stripe_misc::GelatoPhoneReportError>>,
    phone: Option<Option<String>>,
    status: Option<GelatoPhoneReportStatus>,
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

    impl Deserialize for GelatoPhoneReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoPhoneReport>,
        builder: GelatoPhoneReportBuilder,
    }

    impl Visitor for Place<GelatoPhoneReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoPhoneReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoPhoneReportBuilder {
        type Out = GelatoPhoneReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "error" => Deserialize::begin(&mut self.error),
                "phone" => Deserialize::begin(&mut self.phone),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                error: Deserialize::default(),
                phone: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(error), Some(phone), Some(status)) =
                (self.error.take(), self.phone.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { error, phone, status })
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

    impl ObjectDeser for GelatoPhoneReport {
        type Builder = GelatoPhoneReportBuilder;
    }

    impl FromValueOpt for GelatoPhoneReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoPhoneReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "error" => b.error = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `phone` check.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoPhoneReportStatus {
    Unverified,
    Verified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoPhoneReportStatus {
    pub fn as_str(&self) -> &str {
        use GelatoPhoneReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoPhoneReportStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoPhoneReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoPhoneReportStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoPhoneReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoPhoneReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoPhoneReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoPhoneReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoPhoneReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoPhoneReportStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoPhoneReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoPhoneReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
