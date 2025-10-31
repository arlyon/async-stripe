/// Result from an id_number check
#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct GelatoIdNumberReportBuilder {
    dob: Option<Option<stripe_misc::GelatoDataIdNumberReportDate>>,
    error: Option<Option<stripe_misc::GelatoIdNumberReportError>>,
    first_name: Option<Option<String>>,
    id_number: Option<Option<String>>,
    id_number_type: Option<Option<GelatoIdNumberReportIdNumberType>>,
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
                "dob" => Deserialize::begin(&mut self.dob),
                "error" => Deserialize::begin(&mut self.error),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "id_number" => Deserialize::begin(&mut self.id_number),
                "id_number_type" => Deserialize::begin(&mut self.id_number_type),
                "last_name" => Deserialize::begin(&mut self.last_name),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dob: Deserialize::default(),
                error: Deserialize::default(),
                first_name: Deserialize::default(),
                id_number: Deserialize::default(),
                id_number_type: Deserialize::default(),
                last_name: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(dob),
                Some(error),
                Some(first_name),
                Some(id_number),
                Some(id_number_type),
                Some(last_name),
                Some(status),
            ) = (
                self.dob,
                self.error.take(),
                self.first_name.take(),
                self.id_number.take(),
                self.id_number_type,
                self.last_name.take(),
                self.status,
            )
            else {
                return None;
            };
            Some(Self::Out { dob, error, first_name, id_number, id_number_type, last_name, status })
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

    impl ObjectDeser for GelatoIdNumberReport {
        type Builder = GelatoIdNumberReportBuilder;
    }

    impl FromValueOpt for GelatoIdNumberReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoIdNumberReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "first_name" => b.first_name = FromValueOpt::from_value(v),
                    "id_number" => b.id_number = FromValueOpt::from_value(v),
                    "id_number_type" => b.id_number_type = FromValueOpt::from_value(v),
                    "last_name" => b.last_name = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of ID number.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}
impl GelatoIdNumberReportIdNumberType {
    pub fn as_str(self) -> &'static str {
        use GelatoIdNumberReportIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportIdNumberType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        self.out =
            Some(GelatoIdNumberReportIdNumberType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoIdNumberReportIdNumberType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoIdNumberReportIdNumberType")
        })
    }
}
/// Status of this `id_number` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoIdNumberReportStatus {
    Unverified,
    Verified,
}
impl GelatoIdNumberReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoIdNumberReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        self.out = Some(GelatoIdNumberReportStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoIdNumberReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoIdNumberReportStatus"))
    }
}
