/// Result from a document check
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// Array of [File](https://docs.stripe.com/api/files) ids containing images for this document.
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoDocumentReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoDocumentReport").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: GelatoDocumentReportBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "dob" => Deserialize::begin(&mut self.builder.dob),
                "error" => Deserialize::begin(&mut self.builder.error),
                "expiration_date" => Deserialize::begin(&mut self.builder.expiration_date),
                "files" => Deserialize::begin(&mut self.builder.files),
                "first_name" => Deserialize::begin(&mut self.builder.first_name),
                "issued_date" => Deserialize::begin(&mut self.builder.issued_date),
                "issuing_country" => Deserialize::begin(&mut self.builder.issuing_country),
                "last_name" => Deserialize::begin(&mut self.builder.last_name),
                "number" => Deserialize::begin(&mut self.builder.number),
                "sex" => Deserialize::begin(&mut self.builder.sex),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "unparsed_place_of_birth" => {
                    Deserialize::begin(&mut self.builder.unparsed_place_of_birth)
                }
                "unparsed_sex" => Deserialize::begin(&mut self.builder.unparsed_sex),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.address.take(),
                self.builder.dob,
                self.builder.error.take(),
                self.builder.expiration_date,
                self.builder.files.take(),
                self.builder.first_name.take(),
                self.builder.issued_date,
                self.builder.issuing_country.take(),
                self.builder.last_name.take(),
                self.builder.number.take(),
                self.builder.sex.take(),
                self.builder.status.take(),
                self.builder.type_.take(),
                self.builder.unparsed_place_of_birth.take(),
                self.builder.unparsed_sex.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(GelatoDocumentReport {
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
            });
            Ok(())
        }
    }
};
/// Sex of the person in the document.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum GelatoDocumentReportSex {
    Redacted,
    Female,
    Male,
    Unknown,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    /// This variant is prefixed with an underscore to avoid conflicts with Stripe's 'Unknown' variant.
    _Unknown(String),
}
impl GelatoDocumentReportSex {
    pub fn as_str(&self) -> &str {
        use GelatoDocumentReportSex::*;
        match self {
            Redacted => "[redacted]",
            Female => "female",
            Male => "male",
            Unknown => "unknown",
            _Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportSex {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportSex::*;
        match s {
            "[redacted]" => Ok(Redacted),
            "female" => Ok(Female),
            "male" => Ok(Male),
            "unknown" => Ok(Unknown),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "GelatoDocumentReportSex");
                Ok(_Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for GelatoDocumentReportSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoDocumentReportSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoDocumentReportSex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoDocumentReportSex)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for GelatoDocumentReportSex {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoDocumentReportSex> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportSex::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportSex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoDocumentReportStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for GelatoDocumentReportStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoDocumentReportStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(GelatoDocumentReportType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for GelatoDocumentReportType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<GelatoDocumentReportType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoDocumentReportType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
