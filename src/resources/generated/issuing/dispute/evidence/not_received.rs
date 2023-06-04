#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NotReceived {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<crate::Expandable<crate::file::File>>,
    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<crate::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<NotReceivedProductType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NotReceived {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
