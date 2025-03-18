// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::PlatformTaxFeeId;
use crate::params::Object;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PlatformTax".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformTaxFee {
    /// Unique identifier for the object.
    pub id: PlatformTaxFeeId,

    /// The Connected account that incurred this charge.
    pub account: String,

    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,

    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}

impl Object for PlatformTaxFee {
    type Id = PlatformTaxFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "platform_tax_fee"
    }
}
