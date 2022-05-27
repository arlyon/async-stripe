// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{Address};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "UFAResourceBillingDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UfaResourceBillingDetails {

    pub address: Address,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
