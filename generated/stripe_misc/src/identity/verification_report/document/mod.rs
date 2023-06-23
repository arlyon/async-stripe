/// Result from a document check.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Document {
    /// Address as it appears in the document.
pub address: Option<stripe_types::address::Address>,
    /// Date of birth as it appears in the document.
pub dob: Option<stripe_misc::identity::verification_report::document::date_of_birth::DateOfBirth>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
pub error: Option<stripe_misc::identity::verification_report::document::document_check_error::DocumentCheckError>,
    /// Expiration date of the document.
pub expiration_date: Option<stripe_misc::identity::verification_report::document::expiration_date::ExpirationDate>,
    /// Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
pub files: Option<Vec<String>>,
    /// First name as it appears in the document.
pub first_name: Option<String>,
    /// Issued date of the document.
pub issued_date: Option<stripe_misc::identity::verification_report::document::issued_date::IssuedDate>,
    /// Issuing country of the document.
pub issuing_country: Option<String>,
    /// Last name as it appears in the document.
pub last_name: Option<String>,
    /// Document ID number.
pub number: Option<String>,
    /// Status of this `document` check.
pub status: DocumentStatus,
    /// Type of the document.
#[serde(rename = "type")]
pub type_: Option<DocumentType>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Document {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Status of this `document` check.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Unverified,
    Verified,
}

impl DocumentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for DocumentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of the document.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    DrivingLicense,
    IdCard,
    Passport,
}

impl DocumentType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for DocumentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod expiration_date;
pub use expiration_date::ExpirationDate;
pub mod issued_date;
pub use issued_date::IssuedDate;
pub mod document_check_error;
pub use document_check_error::DocumentCheckError;
