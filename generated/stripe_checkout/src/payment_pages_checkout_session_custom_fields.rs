#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    pub label: stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionCustomFieldsType,
}
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}
impl PaymentPagesCheckoutSessionCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomFieldsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionCustomFieldsType",
            )
        })
    }
}
