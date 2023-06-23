#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Other {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_core::file::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<OtherProductType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Other {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum OtherProductType {
    Merchandise,
    Service,
}

impl OtherProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for OtherProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
