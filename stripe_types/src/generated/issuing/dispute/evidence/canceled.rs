#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Canceled {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_types::file::File>>,
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
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CanceledProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merchandise" => Ok(Self::Merchandise),
            "service" => Ok(Self::Service),

            _ => Err(()),
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
impl serde::Serialize for CanceledProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CanceledProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CanceledProductType"))
    }
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CanceledReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merchant_rejected" => Ok(Self::MerchantRejected),
            "successful" => Ok(Self::Successful),

            _ => Err(()),
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
impl serde::Serialize for CanceledReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CanceledReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CanceledReturnStatus"))
    }
}
