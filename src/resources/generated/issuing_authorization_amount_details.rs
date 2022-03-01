// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationAmountDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atm_fee: Option<Box<i64>>,
}
