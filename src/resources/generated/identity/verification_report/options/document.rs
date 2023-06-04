#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Document {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<DocumentAllowedTypes>>,
    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,
    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,
    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    ///
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Document {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Array of strings of allowed identity document types.
///
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl DocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for DocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
