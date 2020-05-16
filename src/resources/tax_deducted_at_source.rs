// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxDeductedAtSource".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxDeductedAtSource {
    /// The end of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: Timestamp,

    /// The start of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: Timestamp,

    /// The TAN that was supplied to Stripe when TDS was assessed.
    pub tax_deduction_account_number: String,
}

impl Object for TaxDeductedAtSource {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "tax_deducted_at_source"
    }
}
