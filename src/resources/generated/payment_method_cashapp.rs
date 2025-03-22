// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_cashapp".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCashapp {

    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,

    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}
