#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerTax {
    /// Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: CustomerTaxAutomaticTax,
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    /// The customer's location as identified by Stripe Tax.
    pub location: Option<stripe_types::CustomerTaxLocation>,
}
/// Surfaces if automatic tax computation is possible given the current customer location information.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerTaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
}

impl CustomerTaxAutomaticTax {
    pub fn as_str(self) -> &'static str {
        use CustomerTaxAutomaticTax::*;
        match self {
            Failed => "failed",
            NotCollecting => "not_collecting",
            Supported => "supported",
            UnrecognizedLocation => "unrecognized_location",
        }
    }
}

impl std::str::FromStr for CustomerTaxAutomaticTax {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxAutomaticTax::*;
        match s {
            "failed" => Ok(Failed),
            "not_collecting" => Ok(NotCollecting),
            "supported" => Ok(Supported),
            "unrecognized_location" => Ok(UnrecognizedLocation),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerTaxAutomaticTax {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerTaxAutomaticTax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTaxAutomaticTax {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerTaxAutomaticTax"))
    }
}
