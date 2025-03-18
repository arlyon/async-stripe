// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::Address;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "billing_details".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<Address>,

    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub name: Option<String>,

    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
