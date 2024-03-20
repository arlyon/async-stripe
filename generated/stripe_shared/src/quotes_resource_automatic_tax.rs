#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotesResourceAutomaticTax {
    /// Automatically calculate taxes
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
    /// The status of the most recent automated tax calculation for this quote.
    pub status: Option<QuotesResourceAutomaticTaxStatus>,
}
/// The status of the most recent automated tax calculation for this quote.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuotesResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}
impl QuotesResourceAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        use QuotesResourceAutomaticTaxStatus::*;
        match self {
            Complete => "complete",
            Failed => "failed",
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for QuotesResourceAutomaticTaxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceAutomaticTaxStatus::*;
        match s {
            "complete" => Ok(Complete),
            "failed" => Ok(Failed),
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for QuotesResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuotesResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for QuotesResourceAutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotesResourceAutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for QuotesResourceAutomaticTaxStatus")
        })
    }
}
