#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoEmailReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoEmailReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoEmailReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoEmailReportError").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoEmailReportErrorBuilder {
    code: Option<Option<GelatoEmailReportErrorCode>>,
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

    impl Deserialize for GelatoEmailReportError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoEmailReportError>,
        builder: GelatoEmailReportErrorBuilder,
    }

    impl Visitor for Place<GelatoEmailReportError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoEmailReportErrorBuilder {
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
            *self.out = Some(GelatoEmailReportError { code, reason });
            Ok(())
        }
    }
};
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoEmailReportErrorCode {
    EmailUnverifiedOther,
    EmailVerificationDeclined,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoEmailReportErrorCode {
    pub fn as_str(&self) -> &str {
        use GelatoEmailReportErrorCode::*;
        match self {
            EmailUnverifiedOther => "email_unverified_other",
            EmailVerificationDeclined => "email_verification_declined",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoEmailReportErrorCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoEmailReportErrorCode::*;
        match s {
            "email_unverified_other" => Ok(EmailUnverifiedOther),
            "email_verification_declined" => Ok(EmailVerificationDeclined),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoEmailReportErrorCode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoEmailReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoEmailReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoEmailReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoEmailReportErrorCode)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoEmailReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for GelatoEmailReportErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoEmailReportErrorCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoEmailReportErrorCode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoEmailReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
