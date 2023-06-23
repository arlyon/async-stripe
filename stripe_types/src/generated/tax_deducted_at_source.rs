#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: stripe_types::tax_deducted_at_source::TaxDeductedAtSourceId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TaxDeductedAtSourceObject,
    /// The end of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: stripe_types::Timestamp,
    /// The start of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: stripe_types::Timestamp,
    /// The TAN that was supplied to Stripe when TDS was assessed.
    pub tax_deduction_account_number: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxDeductedAtSource {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TaxDeductedAtSourceObject {
    TaxDeductedAtSource,
}

impl TaxDeductedAtSourceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaxDeductedAtSource => "tax_deducted_at_source",
        }
    }
}

impl AsRef<str> for TaxDeductedAtSourceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxDeductedAtSourceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for TaxDeductedAtSource {
    type Id = stripe_types::tax_deducted_at_source::TaxDeductedAtSourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxDeductedAtSourceId, "itds_");
