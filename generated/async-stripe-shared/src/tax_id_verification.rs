#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxIdVerification {
    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: TaxIdVerificationStatus,
    /// Verified address.
    pub verified_address: Option<String>,
    /// Verified name.
    pub verified_name: Option<String>,
}
#[doc(hidden)]
pub struct TaxIdVerificationBuilder {
    status: Option<TaxIdVerificationStatus>,
    verified_address: Option<Option<String>>,
    verified_name: Option<Option<String>>,
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

    impl Deserialize for TaxIdVerification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxIdVerification>,
        builder: TaxIdVerificationBuilder,
    }

    impl Visitor for Place<TaxIdVerification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxIdVerificationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxIdVerificationBuilder {
        type Out = TaxIdVerification;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.status),
                "verified_address" => Deserialize::begin(&mut self.verified_address),
                "verified_name" => Deserialize::begin(&mut self.verified_name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                status: Deserialize::default(),
                verified_address: Deserialize::default(),
                verified_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(status), Some(verified_address), Some(verified_name)) =
                (self.status.take(), self.verified_address.take(), self.verified_name.take())
            else {
                return None;
            };
            Some(Self::Out { status, verified_address, verified_name })
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

    impl ObjectDeser for TaxIdVerification {
        type Builder = TaxIdVerificationBuilder;
    }

    impl FromValueOpt for TaxIdVerification {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxIdVerificationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "status" => b.status = FromValueOpt::from_value(v),
                    "verified_address" => b.verified_address = FromValueOpt::from_value(v),
                    "verified_name" => b.verified_name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxIdVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxIdVerificationStatus {
    pub fn as_str(&self) -> &str {
        use TaxIdVerificationStatus::*;
        match self {
            Pending => "pending",
            Unavailable => "unavailable",
            Unverified => "unverified",
            Verified => "verified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxIdVerificationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIdVerificationStatus::*;
        match s {
            "pending" => Ok(Pending),
            "unavailable" => Ok(Unavailable),
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TaxIdVerificationStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxIdVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxIdVerificationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxIdVerificationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxIdVerificationStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxIdVerificationStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxIdVerificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
