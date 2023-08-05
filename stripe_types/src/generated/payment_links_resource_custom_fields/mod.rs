#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceCustomFields {
    /// Configuration for `type=dropdown` fields.
    pub dropdown: Option<stripe_types::PaymentLinksResourceCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    pub label: stripe_types::PaymentLinksResourceCustomFieldsLabel,
    /// Configuration for `type=numeric` fields.
    pub numeric: Option<stripe_types::PaymentLinksResourceCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    pub optional: bool,
    /// Configuration for `type=text` fields.
    pub text: Option<stripe_types::PaymentLinksResourceCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceCustomFieldsType,
}
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl PaymentLinksResourceCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceCustomFieldsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinksResourceCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinksResourceCustomFieldsType")
        })
    }
}
