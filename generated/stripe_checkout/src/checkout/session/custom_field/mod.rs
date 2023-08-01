#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomField {
    /// Configuration for `type=dropdown` fields.
pub dropdown: Option<stripe_checkout::checkout::session::custom_field::custom_field_dropdown::CustomFieldDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
pub key: String,
pub label: stripe_checkout::checkout::session::custom_field::custom_field_label::CustomFieldLabel,
    /// Configuration for `type=numeric` fields.
pub numeric: Option<stripe_checkout::checkout::session::custom_field::custom_field_numeric::CustomFieldNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
pub optional: bool,
    /// Configuration for `type=text` fields.
pub text: Option<stripe_checkout::checkout::session::custom_field::custom_field_text::CustomFieldText>,
    /// The type of the field.
#[serde(rename = "type")]
pub type_: CustomFieldType,

}
/// The type of the field.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomFieldType {
    Dropdown,
    Numeric,
    Text,
}

impl CustomFieldType {
    pub fn as_str(self) -> &'static str {
        use CustomFieldType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for CustomFieldType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomFieldType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomFieldType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomFieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CustomFieldType"))
    }
}
pub mod custom_field_dropdown;
pub use custom_field_dropdown::CustomFieldDropdown;
pub mod custom_field_label;
pub use custom_field_label::CustomFieldLabel;
pub mod custom_field_numeric;
pub use custom_field_numeric::CustomFieldNumeric;
pub mod custom_field_text;
pub use custom_field_text::CustomFieldText;
