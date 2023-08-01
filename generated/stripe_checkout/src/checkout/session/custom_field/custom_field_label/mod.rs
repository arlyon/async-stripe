#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: Option<String>,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CustomFieldLabelType,
}
/// The type of the label.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomFieldLabelType {
    Custom,
}

impl CustomFieldLabelType {
    pub fn as_str(self) -> &'static str {
        use CustomFieldLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for CustomFieldLabelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomFieldLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomFieldLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomFieldLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomFieldLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomFieldLabelType"))
    }
}
