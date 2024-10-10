// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_card_present_networks".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCardPresentNetworks {
    /// All available networks for the card.
    pub available: Vec<String>,

    /// The preferred network for the card.
    pub preferred: Option<String>,
}
