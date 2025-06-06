// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_paypal".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodPaypal {

    /// Two-letter ISO code representing the buyer's country.
    ///
    /// Values are provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub country: Option<String>,

    /// Owner's email.
    ///
    /// Values are provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub payer_email: Option<String>,

    /// PayPal account PayerID.
    ///
    /// This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}
