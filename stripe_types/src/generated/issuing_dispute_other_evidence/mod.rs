#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingDisputeOtherEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_types::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,
    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeOtherEvidenceProductType>,
}
/// Whether the product was a merchandise or service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeOtherEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeOtherEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeOtherEvidenceProductType::*;
        match self {
            Merchandise => "merchandise",
            Service => "service",
        }
    }
}

impl std::str::FromStr for IssuingDisputeOtherEvidenceProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeOtherEvidenceProductType::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingDisputeOtherEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeOtherEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeOtherEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeOtherEvidenceProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingDisputeOtherEvidenceProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingDisputeOtherEvidenceProductType")
        })
    }
}
