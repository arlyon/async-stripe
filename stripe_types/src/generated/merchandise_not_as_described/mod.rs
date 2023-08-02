#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MerchandiseNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<stripe_types::Timestamp>,
    /// Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,
    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<MerchandiseNotAsDescribedReturnStatus>,
    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<stripe_types::Timestamp>,
}
/// Result of cardholder's attempt to return the product.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MerchandiseNotAsDescribedReturnStatus {
    MerchantRejected,
    Successful,
}

impl MerchandiseNotAsDescribedReturnStatus {
    pub fn as_str(self) -> &'static str {
        use MerchandiseNotAsDescribedReturnStatus::*;
        match self {
            MerchantRejected => "merchant_rejected",
            Successful => "successful",
        }
    }
}

impl std::str::FromStr for MerchandiseNotAsDescribedReturnStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MerchandiseNotAsDescribedReturnStatus::*;
        match s {
            "merchant_rejected" => Ok(MerchantRejected),
            "successful" => Ok(Successful),
            _ => Err(()),
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
impl serde::Serialize for MerchandiseNotAsDescribedReturnStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MerchandiseNotAsDescribedReturnStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MerchandiseNotAsDescribedReturnStatus")
        })
    }
}
