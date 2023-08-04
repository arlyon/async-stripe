#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The status of the most recent automated tax calculation for this invoice.
    pub status: Option<AutomaticTaxStatus>,
}
/// The status of the most recent automated tax calculation for this invoice.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl AutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        use AutomaticTaxStatus::*;
        match self {
            Complete => "complete",
            Failed => "failed",
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for AutomaticTaxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AutomaticTaxStatus::*;
        match s {
            "complete" => Ok(Complete),
            "failed" => Ok(Failed),
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AutomaticTaxStatus"))
    }
}
