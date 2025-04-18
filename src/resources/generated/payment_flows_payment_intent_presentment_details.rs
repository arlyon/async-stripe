// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentFlowsPaymentIntentPresentmentDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPaymentIntentPresentmentDetails {

    /// Amount intended to be collected by this payment, denominated in presentment_currency.
    pub presentment_amount: i64,

    /// Currency presented to the customer during payment.
    pub presentment_currency: Currency,
}
