#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoSessionDocumentOptions {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types: Option<Vec<GelatoSessionDocumentOptionsAllowedTypes>>,
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
/// Array of strings of allowed identity document types.
///
/// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSessionDocumentOptionsAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl GelatoSessionDocumentOptionsAllowedTypes {
    pub fn as_str(self) -> &'static str {
        use GelatoSessionDocumentOptionsAllowedTypes::*;
        match self {
            DrivingLicense => "driving_license",
            IdCard => "id_card",
            Passport => "passport",
        }
    }
}

impl std::str::FromStr for GelatoSessionDocumentOptionsAllowedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSessionDocumentOptionsAllowedTypes::*;
        match s {
            "driving_license" => Ok(DrivingLicense),
            "id_card" => Ok(IdCard),
            "passport" => Ok(Passport),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoSessionDocumentOptionsAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSessionDocumentOptionsAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSessionDocumentOptionsAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoSessionDocumentOptionsAllowedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoSessionDocumentOptionsAllowedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoSessionDocumentOptionsAllowedTypes")
        })
    }
}
