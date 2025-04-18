// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_sofort".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodSofort {

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
}
