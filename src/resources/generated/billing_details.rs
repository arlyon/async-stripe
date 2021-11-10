// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::resources::Address;

/// The resource representing a Stripe "billing_details".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingDetails {
    /// Billing address.
    pub address: Box<Option<Address>>,

    /// Email address.
    pub email: Box<Option<String>>,

    /// Full name.
    pub name: Box<Option<String>>,

    /// Billing phone number (including extension).
    pub phone: Box<Option<String>>,
}
