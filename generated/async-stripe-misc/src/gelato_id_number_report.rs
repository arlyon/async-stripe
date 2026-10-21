/// Result from an id_number check
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoIdNumberReport {
    /// Date of birth.
    pub dob: Option<stripe_misc::GelatoDataIdNumberReportDate>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoIdNumberReportError>,
    /// First name.
    pub first_name: Option<String>,
    /// ID number. When `id_number_type` is `us_ssn`, only the last 4 digits are present.
    pub id_number: Option<String>,
    /// Type of ID number.
    pub id_number_type: Option<GelatoIdNumberReportIdNumberType>,
    /// Last name.
    pub last_name: Option<String>,
    /// Status of this `id_number` check.
    pub status: GelatoIdNumberReportStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoIdNumberReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoIdNumberReport").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoIdNumberReportBuilder {
    dob: Option<stripe_misc::GelatoDataIdNumberReportDate>,
    error: Option<stripe_misc::GelatoIdNumberReportError>,
    first_name: Option<Option<String>>,
    id_number: Option<Option<String>>,
    id_number_type: Option<GelatoIdNumberReportIdNumberType>,
    last_name: Option<Option<String>>,
    status: Option<GelatoIdNumberReportStatus>,
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

    impl Deserialize for GelatoIdNumberReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoIdNumberReport>,
        builder: GelatoIdNumberReportBuilder,
    }

    impl Visitor for Place<GelatoIdNumberReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoIdNumberReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoIdNumberReportBuilder {
        type Out = GelatoIdNumberReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dob" => stripe_types::miniserde_helpers::begin_nullable(&mut self.dob),
                "error" => stripe_types::miniserde_helpers::begin_nullable(&mut self.error),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "id_number" => Deserialize::begin(&mut self.id_number),
                "id_number_type" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.id_number_type)
                }
                "last_name" => Deserialize::begin(&mut self.last_name),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dob: None,
                error: None,
                first_name: Some(None),
                id_number: Some(None),
                id_number_type: None,
                last_name: Some(None),
                status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    dob,
                    error,
                    first_name: Some(first_name),
                    id_number: Some(id_number),
                    id_number_type,
                    last_name: Some(last_name),
                    status: Some(status),
                } => Some(Self::Out {
                    dob,
                    error,
                    first_name,
                    id_number,
                    id_number_type,
                    last_name,
                    status,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for GelatoIdNumberReport {
        type Builder = GelatoIdNumberReportBuilder;
    }

    impl FromValueOpt for GelatoIdNumberReport {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Type of ID number.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoIdNumberReportIdNumberType {
    pub fn as_str(&self) -> &str {
        use GelatoIdNumberReportIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportIdNumberType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "GelatoIdNumberReportIdNumberType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoIdNumberReportIdNumberType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoIdNumberReportIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoIdNumberReportIdNumberType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoIdNumberReportIdNumberType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoIdNumberReportIdNumberType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoIdNumberReportIdNumberType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Status of this `id_number` check.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoIdNumberReportStatus {
    Unverified,
    Verified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoIdNumberReportStatus {
    pub fn as_str(&self) -> &str {
        use GelatoIdNumberReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoIdNumberReportStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoIdNumberReportStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoIdNumberReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoIdNumberReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoIdNumberReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoIdNumberReportStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoIdNumberReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
