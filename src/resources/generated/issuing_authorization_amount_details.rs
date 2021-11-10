// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationAmountDetails".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorizationAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Box<Option<i64>>,
}
