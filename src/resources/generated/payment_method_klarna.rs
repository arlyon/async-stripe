// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_klarna".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodKlarna {

    /// The customer's date of birth, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {

    /// The day of birth, between 1 and 31.
    pub day: Option<i64>,

    /// The month of birth, between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year of birth.
    pub year: Option<i64>,
}
