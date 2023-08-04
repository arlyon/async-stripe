#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    /// Indicates whether automatic tax is enabled for the session.
    pub enabled: bool,
    /// The status of the most recent automated tax calculation for this session.
    pub status: Option<PaymentPagesCheckoutSessionAutomaticTaxStatus>,
}
/// The status of the most recent automated tax calculation for this session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl PaymentPagesCheckoutSessionAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionAutomaticTaxStatus::*;
        match self {
            Complete => "complete",
            Failed => "failed",
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionAutomaticTaxStatus::*;
        match s {
            "complete" => Ok(Complete),
            "failed" => Ok(Failed),
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionAutomaticTaxStatus",
            )
        })
    }
}
