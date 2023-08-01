#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SellerProtection {
    /// An array of conditions that are covered for the transaction, if applicable.
    pub dispute_categories: Option<Vec<SellerProtectionDisputeCategories>>,
    /// Indicates whether the transaction is eligible for PayPal's seller protection.
    pub status: SellerProtectionStatus,
}
/// An array of conditions that are covered for the transaction, if applicable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SellerProtectionDisputeCategories {
    Fraudulent,
    ProductNotReceived,
}

impl SellerProtectionDisputeCategories {
    pub fn as_str(self) -> &'static str {
        use SellerProtectionDisputeCategories::*;
        match self {
            Fraudulent => "fraudulent",
            ProductNotReceived => "product_not_received",
        }
    }
}

impl std::str::FromStr for SellerProtectionDisputeCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SellerProtectionDisputeCategories::*;
        match s {
            "fraudulent" => Ok(Fraudulent),
            "product_not_received" => Ok(ProductNotReceived),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SellerProtectionDisputeCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SellerProtectionDisputeCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SellerProtectionDisputeCategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SellerProtectionDisputeCategories")
        })
    }
}
/// Indicates whether the transaction is eligible for PayPal's seller protection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SellerProtectionStatus {
    Eligible,
    NotEligible,
    PartiallyEligible,
}

impl SellerProtectionStatus {
    pub fn as_str(self) -> &'static str {
        use SellerProtectionStatus::*;
        match self {
            Eligible => "eligible",
            NotEligible => "not_eligible",
            PartiallyEligible => "partially_eligible",
        }
    }
}

impl std::str::FromStr for SellerProtectionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SellerProtectionStatus::*;
        match s {
            "eligible" => Ok(Eligible),
            "not_eligible" => Ok(NotEligible),
            "partially_eligible" => Ok(PartiallyEligible),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SellerProtectionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SellerProtectionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SellerProtectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SellerProtectionStatus"))
    }
}
