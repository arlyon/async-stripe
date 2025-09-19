#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct VerificationSessionRedaction {
    /// Indicates whether this object and its related objects have been redacted or not.
    pub status: VerificationSessionRedactionStatus,
}
#[doc(hidden)]
pub struct VerificationSessionRedactionBuilder {
    status: Option<VerificationSessionRedactionStatus>,
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

    impl Deserialize for VerificationSessionRedaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<VerificationSessionRedaction>,
        builder: VerificationSessionRedactionBuilder,
    }

    impl Visitor for Place<VerificationSessionRedaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: VerificationSessionRedactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for VerificationSessionRedactionBuilder {
        type Out = VerificationSessionRedaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(status),) = (self.status,) else {
                return None;
            };
            Some(Self::Out { status })
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

    impl ObjectDeser for VerificationSessionRedaction {
        type Builder = VerificationSessionRedactionBuilder;
    }

    impl FromValueOpt for VerificationSessionRedaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = VerificationSessionRedactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates whether this object and its related objects have been redacted or not.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VerificationSessionRedactionStatus {
    Processing,
    Redacted,
}
impl VerificationSessionRedactionStatus {
    pub fn as_str(self) -> &'static str {
        use VerificationSessionRedactionStatus::*;
        match self {
            Processing => "processing",
            Redacted => "redacted",
        }
    }
}

impl std::str::FromStr for VerificationSessionRedactionStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use VerificationSessionRedactionStatus::*;
        match s {
            "processing" => Ok(Processing),
            "redacted" => Ok(Redacted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for VerificationSessionRedactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for VerificationSessionRedactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for VerificationSessionRedactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for VerificationSessionRedactionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<VerificationSessionRedactionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(VerificationSessionRedactionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(VerificationSessionRedactionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for VerificationSessionRedactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for VerificationSessionRedactionStatus")
        })
    }
}
