#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: stripe_types::tax_deducted_at_source::TaxDeductedAtSourceId,
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
impl stripe_types::Object for TaxDeductedAtSource {
    type Id = stripe_types::tax_deducted_at_source::TaxDeductedAtSourceId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TaxDeductedAtSourceId, "itds_");
