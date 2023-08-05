#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: Option<String>,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceCustomFieldsLabelType,
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceCustomFieldsLabelType {
    Custom,
}

impl PaymentLinksResourceCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceCustomFieldsLabelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinksResourceCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinksResourceCustomFieldsLabelType")
        })
    }
}
