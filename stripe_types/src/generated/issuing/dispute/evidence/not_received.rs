#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NotReceived {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<NotReceivedProductType>,
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NotReceivedProductType {
    Merchandise,
    Service,
}

impl NotReceivedProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl std::str::FromStr for NotReceivedProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merchandise" => Ok(Self::Merchandise),
            "service" => Ok(Self::Service),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for NotReceivedProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for NotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for NotReceivedProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NotReceivedProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for NotReceivedProductType"))
    }
}
