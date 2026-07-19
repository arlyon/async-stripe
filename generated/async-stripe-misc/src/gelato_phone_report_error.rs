#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoPhoneReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoPhoneReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoPhoneReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoPhoneReportError").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoPhoneReportErrorBuilder {
    code: Option<Option<GelatoPhoneReportErrorCode>>,
    reason: Option<Option<String>>,
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
                builder: GelatoPhoneReportErrorBuilder {
                    code: Deserialize::default(),
                    reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.builder.code),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(code), Some(reason)) = (self.builder.code.take(), self.builder.reason.take())
            else {
                return Ok(());
            };
            *self.out = Some(GelatoPhoneReportError { code, reason });
            Ok(())
        }
    }
};
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoPhoneReportErrorCode {
    PhoneUnverifiedOther,
    PhoneVerificationDeclined,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoPhoneReportErrorCode {
    pub fn as_str(&self) -> &str {
        use GelatoPhoneReportErrorCode::*;
        match self {
            PhoneUnverifiedOther => "phone_unverified_other",
            PhoneVerificationDeclined => "phone_verification_declined",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoPhoneReportErrorCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoPhoneReportErrorCode::*;
        match s {
            "phone_unverified_other" => Ok(PhoneUnverifiedOther),
            "phone_verification_declined" => Ok(PhoneVerificationDeclined),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoPhoneReportErrorCode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoPhoneReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoPhoneReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoPhoneReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoPhoneReportErrorCode)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for GelatoPhoneReportErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoPhoneReportErrorCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoPhoneReportErrorCode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoPhoneReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
