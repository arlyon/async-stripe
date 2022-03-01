// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;

/// The resource representing a Stripe "PlatformTax".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,

    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,

    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}

impl Object for PlatformTaxFee {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "platform_tax_fee"
    }
}
