#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Tax {
    /// Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: TaxAutomaticTax,
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    /// The customer's location as identified by Stripe Tax.
    pub location: Option<stripe_core::customer::tax::location::Location>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Tax {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Surfaces if automatic tax computation is possible given the current customer location information.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
}

impl TaxAutomaticTax {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::NotCollecting => "not_collecting",
            Self::Supported => "supported",
            Self::UnrecognizedLocation => "unrecognized_location",
        }
    }
}

impl AsRef<str> for TaxAutomaticTax {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod location;
pub use location::Location;
