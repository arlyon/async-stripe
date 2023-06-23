#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Canceled {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_core::file::File>>,
    /// Date when order was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Whether the cardholder was provided with a cancellation policy.
    pub cancellation_policy_provided: Option<bool>,
    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<stripe_types::Timestamp>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<CanceledProductType>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<CanceledReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Canceled {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CanceledProductType {
    Merchandise,
    Service,
}

impl CanceledProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for CanceledProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CanceledReturnStatus {
    MerchantRejected,
    Successful,
}

impl CanceledReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for CanceledReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
