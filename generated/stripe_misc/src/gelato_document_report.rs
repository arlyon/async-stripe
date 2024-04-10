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
    /// Status of this `document` check.
    pub status: GelatoDocumentReportStatus,
    /// Type of the document.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<GelatoDocumentReportType>,
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
    status: Option<GelatoDocumentReportStatus>,
    type_: Option<Option<GelatoDocumentReportType>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

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
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                address: self.address.take()?,
                dob: self.dob?,
                error: self.error.take()?,
                expiration_date: self.expiration_date?,
                files: self.files.take()?,
                first_name: self.first_name.take()?,
                issued_date: self.issued_date?,
                issuing_country: self.issuing_country.take()?,
                last_name: self.last_name.take()?,
                number: self.number.take()?,
                status: self.status?,
                type_: self.type_?,
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
                    "address" => b.address = Some(FromValueOpt::from_value(v)?),
                    "dob" => b.dob = Some(FromValueOpt::from_value(v)?),
                    "error" => b.error = Some(FromValueOpt::from_value(v)?),
                    "expiration_date" => b.expiration_date = Some(FromValueOpt::from_value(v)?),
                    "files" => b.files = Some(FromValueOpt::from_value(v)?),
                    "first_name" => b.first_name = Some(FromValueOpt::from_value(v)?),
                    "issued_date" => b.issued_date = Some(FromValueOpt::from_value(v)?),
                    "issuing_country" => b.issuing_country = Some(FromValueOpt::from_value(v)?),
                    "last_name" => b.last_name = Some(FromValueOpt::from_value(v)?),
                    "number" => b.number = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of this `document` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
}
impl GelatoDocumentReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoDocumentReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
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
        self.out = Some(GelatoDocumentReportStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoDocumentReportStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoDocumentReportStatus"))
    }
}
/// Type of the document.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
}
impl GelatoDocumentReportType {
    pub fn as_str(self) -> &'static str {
        use GelatoDocumentReportType::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for GelatoDocumentReportType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoDocumentReportType::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
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
        self.out = Some(GelatoDocumentReportType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoDocumentReportType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoDocumentReportType"))
    }
}
