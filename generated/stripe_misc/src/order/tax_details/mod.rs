#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxDetails {
    /// Describes the purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    pub tax_exempt: TaxDetailsTaxExempt,
    /// The purchaser's tax IDs to be used in calculation of tax for this Order.
    pub tax_ids: Vec<stripe_misc::order::tax_details::tax_id::TaxId>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Describes the purchaser's tax exemption status.
///
/// One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl TaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for TaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod tax_id;
pub use tax_id::TaxId;
