#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoPhoneReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoPhoneReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[doc(hidden)]
pub struct GelatoPhoneReportErrorBuilder {
    code: Option<Option<GelatoPhoneReportErrorCode>>,
    reason: Option<Option<String>>,
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

    impl Deserialize for GelatoPhoneReportError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoPhoneReportError>,
        builder: GelatoPhoneReportErrorBuilder,
    }

    impl Visitor for Place<GelatoPhoneReportError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoPhoneReportErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoPhoneReportErrorBuilder {
        type Out = GelatoPhoneReportError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "reason" => Deserialize::begin(&mut self.reason),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default(), reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(reason)) = (self.code, self.reason.take()) else {
                return None;
            };
            Some(Self::Out { code, reason })
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

    impl ObjectDeser for GelatoPhoneReportError {
        type Builder = GelatoPhoneReportErrorBuilder;
    }

    impl FromValueOpt for GelatoPhoneReportError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoPhoneReportErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoPhoneReportErrorCode {
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
}
impl GelatoPhoneReportErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoPhoneReportErrorCode::*;
        match self {
            PhoneUnverifiedOther => "phone_unverified_other",
            PhoneVerificationDeclined => "phone_verification_declined",
        }
    }
}

impl std::str::FromStr for GelatoPhoneReportErrorCode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoPhoneReportErrorCode::*;
        match s {
            "phone_unverified_other" => Ok(PhoneUnverifiedOther),
            "phone_verification_declined" => Ok(PhoneVerificationDeclined),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoPhoneReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoPhoneReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoPhoneReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoPhoneReportErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoPhoneReportErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoPhoneReportErrorCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoPhoneReportErrorCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoPhoneReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoPhoneReportErrorCode"))
    }
}
