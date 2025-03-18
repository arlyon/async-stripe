// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TaxDeductedAtSourceId;
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxDeductedAtSource".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: TaxDeductedAtSourceId,

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
    type Id = TaxDeductedAtSourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_deducted_at_source"
    }
}
