// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::resources::Address;

/// The resource representing a Stripe "billing_details".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
