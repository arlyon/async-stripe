#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MerchandiseNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<crate::Expandable<crate::file::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<crate::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<MerchandiseNotAsDescribedReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<crate::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MerchandiseNotAsDescribed {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MerchandiseNotAsDescribedReturnStatus {
    MerchantRejected,
    Successful,
}

impl MerchandiseNotAsDescribedReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for MerchandiseNotAsDescribedReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MerchandiseNotAsDescribedReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
