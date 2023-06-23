#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The status of the most recent automated tax calculation for this invoice.
    pub status: Option<AutomaticTaxStatus>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AutomaticTax {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The status of the most recent automated tax calculation for this invoice.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl AutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::RequiresLocationInputs => "requires_location_inputs",
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
        self.as_str().fmt(f)
    }
}
