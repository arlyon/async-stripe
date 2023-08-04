#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: Option<String>,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionCustomFieldsLabelType,
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomFieldsLabelType {
    Custom,
}

impl PaymentPagesCheckoutSessionCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionCustomFieldsLabelType",
            )
        })
    }
}
