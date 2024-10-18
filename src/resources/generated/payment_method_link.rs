// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_link".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodLink {

    /// Account owner's email address.
    pub email: Option<String>,

    /// [Deprecated] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}
