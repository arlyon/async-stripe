#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaypalSellerProtection {
    /// An array of conditions that are covered for the transaction, if applicable.
    pub dispute_categories: Option<Vec<PaypalSellerProtectionDisputeCategories>>,
    /// Indicates whether the transaction is eligible for PayPal's seller protection.
    pub status: PaypalSellerProtectionStatus,
}
/// An array of conditions that are covered for the transaction, if applicable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaypalSellerProtectionDisputeCategories {
    Fraudulent,
    ProductNotReceived,
}

impl PaypalSellerProtectionDisputeCategories {
    pub fn as_str(self) -> &'static str {
        use PaypalSellerProtectionDisputeCategories::*;
        match self {
            Fraudulent => "fraudulent",
            ProductNotReceived => "product_not_received",
        }
    }
}

impl std::str::FromStr for PaypalSellerProtectionDisputeCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaypalSellerProtectionDisputeCategories::*;
        match s {
            "fraudulent" => Ok(Fraudulent),
            "product_not_received" => Ok(ProductNotReceived),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaypalSellerProtectionDisputeCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaypalSellerProtectionDisputeCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionDisputeCategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaypalSellerProtectionDisputeCategories"))
    }
}
/// Indicates whether the transaction is eligible for PayPal's seller protection.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaypalSellerProtectionStatus {
    Eligible,
    NotEligible,
    PartiallyEligible,
}

impl PaypalSellerProtectionStatus {
    pub fn as_str(self) -> &'static str {
        use PaypalSellerProtectionStatus::*;
        match self {
            Eligible => "eligible",
            NotEligible => "not_eligible",
            PartiallyEligible => "partially_eligible",
        }
    }
}

impl std::str::FromStr for PaypalSellerProtectionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaypalSellerProtectionStatus::*;
        match s {
            "eligible" => Ok(Eligible),
            "not_eligible" => Ok(NotEligible),
            "partially_eligible" => Ok(PartiallyEligible),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaypalSellerProtectionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaypalSellerProtectionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaypalSellerProtectionStatus"))
    }
}
