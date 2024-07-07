/// Result from a selfie check
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSelfieReport {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoSelfieReportError>,
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    /// Status of this `selfie` check.
    pub status: GelatoSelfieReportStatus,
}
#[doc(hidden)]
pub struct GelatoSelfieReportBuilder {
    document: Option<Option<String>>,
    error: Option<Option<stripe_misc::GelatoSelfieReportError>>,
    selfie: Option<Option<String>>,
    status: Option<GelatoSelfieReportStatus>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoSelfieReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSelfieReport>,
        builder: GelatoSelfieReportBuilder,
    }

    impl Visitor for Place<GelatoSelfieReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSelfieReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSelfieReportBuilder {
        type Out = GelatoSelfieReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.document),
                "error" => Deserialize::begin(&mut self.error),
                "selfie" => Deserialize::begin(&mut self.selfie),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                document: Deserialize::default(),
                error: Deserialize::default(),
                selfie: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                document: self.document.take()?,
                error: self.error.take()?,
                selfie: self.selfie.take()?,
                status: self.status?,
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

    impl ObjectDeser for GelatoSelfieReport {
        type Builder = GelatoSelfieReportBuilder;
    }

    impl FromValueOpt for GelatoSelfieReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSelfieReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "document" => b.document = Some(FromValueOpt::from_value(v)?),
                    "error" => b.error = Some(FromValueOpt::from_value(v)?),
                    "selfie" => b.selfie = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `selfie` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSelfieReportStatus {
    Unverified,
    Verified,
}
impl GelatoSelfieReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoSelfieReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoSelfieReportStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSelfieReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoSelfieReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoSelfieReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoSelfieReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSelfieReportStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoSelfieReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSelfieReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoSelfieReportStatus"))
    }
}
