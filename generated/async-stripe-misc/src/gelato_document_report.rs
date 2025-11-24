/// Result from a document check
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoDocumentReport {
    /// Address as it appears in the document.
    pub address: Option<stripe_shared::Address>,
    /// Date of birth as it appears in the document.
    pub dob: Option<stripe_misc::GelatoDataDocumentReportDateOfBirth>,
    /// Details on the verification error. Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoDocumentReportError>,
    /// Expiration date of the document.
    pub expiration_date: Option<stripe_misc::GelatoDataDocumentReportExpirationDate>,
    /// Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
    pub files: Option<Vec<String>>,
    /// First name as it appears in the document.
    pub first_name: Option<String>,
    /// Issued date of the document.
    pub issued_date: Option<stripe_misc::GelatoDataDocumentReportIssuedDate>,
    /// Issuing country of the document.
    pub issuing_country: Option<String>,
    /// Last name as it appears in the document.
    pub last_name: Option<String>,
    /// Document ID number.
    pub number: Option<String>,
    /// Sex of the person in the document.
    pub sex: Option<GelatoDocumentReportSex>,
    /// Status of this `document` check.
    pub status: GelatoDocumentReportStatus,
    /// Type of the document.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<GelatoDocumentReportType>,
    /// Place of birth as it appears in the document.
    pub unparsed_place_of_birth: Option<String>,
    /// Sex as it appears in the document.
    pub unparsed_sex: Option<String>,
}
#[doc(hidden)]
pub struct GelatoDocumentReportBuilder {
    address: Option<Option<stripe_shared::Address>>,
    dob: Option<Option<stripe_misc::GelatoDataDocumentReportDateOfBirth>>,
    error: Option<Option<stripe_misc::GelatoDocumentReportError>>,
    expiration_date: Option<Option<stripe_misc::GelatoDataDocumentReportExpirationDate>>,
    files: Option<Option<Vec<String>>>,
    first_name: Option<Option<String>>,
    issued_date: Option<Option<stripe_misc::GelatoDataDocumentReportIssuedDate>>,
    issuing_country: Option<Option<String>>,
    last_name: Option<Option<String>>,
    number: Option<Option<String>>,
    sex: Option<Option<GelatoDocumentReportSex>>,
    status: Option<GelatoDocumentReportStatus>,
    type_: Option<Option<GelatoDocumentReportType>>,
    unparsed_place_of_birth: Option<Option<String>>,
    unparsed_sex: Option<Option<String>>,
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

    impl Deserialize for GelatoDocumentReport {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoDocumentReport>,
        builder: GelatoDocumentReportBuilder,
    }

    impl Visitor for Place<GelatoDocumentReport> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoDocumentReportBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoDocumentReportBuilder {
        type Out = GelatoDocumentReport;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "dob" => Deserialize::begin(&mut self.dob),
                "error" => Deserialize::begin(&mut self.error),
                "expiration_date" => Deserialize::begin(&mut self.expiration_date),
                "files" => Deserialize::begin(&mut self.files),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "issued_date" => Deserialize::begin(&mut self.issued_date),
                "issuing_country" => Deserialize::begin(&mut self.issuing_country),
                "last_name" => Deserialize::begin(&mut self.last_name),
                "number" => Deserialize::begin(&mut self.number),
                "sex" => Deserialize::begin(&mut self.sex),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),
                "unparsed_place_of_birth" => Deserialize::begin(&mut self.unparsed_place_of_birth),
                "unparsed_sex" => Deserialize::begin(&mut self.unparsed_sex),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                dob: Deserialize::default(),
                error: Deserialize::default(),
                expiration_date: Deserialize::default(),
                files: Deserialize::default(),
                first_name: Deserialize::default(),
                issued_date: Deserialize::default(),
                issuing_country: Deserialize::default(),
                last_name: Deserialize::default(),
                number: Deserialize::default(),
                sex: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
                unparsed_place_of_birth: Deserialize::default(),
                unparsed_sex: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(dob),
                Some(error),
                Some(expiration_date),
                Some(files),
                Some(first_name),
                Some(issued_date),
                Some(issuing_country),
                Some(last_name),
                Some(number),
                Some(sex),
                Some(status),
                Some(type_),
                Some(unparsed_place_of_birth),
                Some(unparsed_sex),
            ) = (
                self.address.take(),
                self.dob,
                self.error.take(),
                self.expiration_date,
                self.files.take(),
                self.first_name.take(),
                self.issued_date,
                self.issuing_country.take(),
                self.last_name.take(),
                self.number.take(),
                self.sex,
                self.status.take(),
                self.type_.take(),
                self.unparsed_place_of_birth.take(),
                self.unparsed_sex.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                dob,
                error,
                expiration_date,
                files,
                first_name,
                issued_date,
                issuing_country,
                last_name,
                number,
                sex,
                status,
                type_,
                unparsed_place_of_birth,
                unparsed_sex,
            })
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

    impl ObjectDeser for GelatoDocumentReport {
        type Builder = GelatoDocumentReportBuilder;
    }

    impl FromValueOpt for GelatoDocumentReport {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoDocumentReportBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "dob" => b.dob = FromValueOpt::from_value(v),
                    "error" => b.error = FromValueOpt::from_value(v),
                    "expiration_date" => b.expiration_date = FromValueOpt::from_value(v),
                    "files" => b.files = FromValueOpt::from_value(v),
                    "first_name" => b.first_name = FromValueOpt::from_value(v),
                    "issued_date" => b.issued_date = FromValueOpt::from_value(v),
                    "issuing_country" => b.issuing_country = FromValueOpt::from_value(v),
                    "last_name" => b.last_name = FromValueOpt::from_value(v),
                    "number" => b.number = FromValueOpt::from_value(v),
                    "sex" => b.sex = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unparsed_place_of_birth" => {
                        b.unparsed_place_of_birth = FromValueOpt::from_value(v)
                    }
                    "unparsed_sex" => b.unparsed_sex = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Sex of the person in the document.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoDocumentReportSex {
    Redacted,
    Female,
    Male,
    Unknown,
}
impl GelatoDocumentReportSex {
    pub fn as_str(self) -> &'static str {
        use GelatoDocumentReportSex::*;
        match self {
            Redacted => "[redacted]",
            Female => "female",
            Male => "male",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportSex {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportSex::*;
        match s {
            "[redacted]" => Ok(Redacted),
            "female" => Ok(Female),
            "male" => Ok(Male),
            "unknown" => Ok(Unknown),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoDocumentReportSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoDocumentReportSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoDocumentReportSex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoDocumentReportSex {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoDocumentReportSex> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportSex::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoDocumentReportSex);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportSex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoDocumentReportSex"))
    }
}
/// Status of this `document` check.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoDocumentReportStatus {
    pub fn as_str(&self) -> &str {
        use GelatoDocumentReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoDocumentReportStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoDocumentReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoDocumentReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoDocumentReportStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoDocumentReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of the document.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl GelatoDocumentReportType {
    pub fn as_str(&self) -> &str {
        use GelatoDocumentReportType::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportType::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoDocumentReportType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoDocumentReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoDocumentReportType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoDocumentReportType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoDocumentReportType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
