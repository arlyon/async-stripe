// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_details_card_present_offline".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresentOffline {

    /// Time at which the payment was collected while offline.
    pub stored_at: Option<Timestamp>,
}
