// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{Address};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasurySharedResourceBillingDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasurySharedResourceBillingDetails {

    pub address: Address,

    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub name: Option<String>,
}
