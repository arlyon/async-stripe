// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentFlowsAmountDetailsClientResourceTip".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsAmountDetailsClientResourceTip {

    /// Portion of the amount that corresponds to a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
