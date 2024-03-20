/// Result from a document check
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    #[serde(rename = "type")]
    pub type_: Option<GelatoDocumentReportType>,
}
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
impl serde::Serialize for GelatoDocumentReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
impl serde::Serialize for GelatoDocumentReportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoDocumentReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoDocumentReportType"))
    }
}
