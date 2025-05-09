// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_card_present_routing".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsCardPresentRouting {

    /// Requested routing priority.
    pub requested_priority: Option<PaymentMethodOptionsCardPresentRoutingRequestedPriority>,
}

/// An enum representing the possible values of an `PaymentMethodOptionsCardPresentRouting`'s `requested_priority` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    Domestic,
    International,
}

impl PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsCardPresentRoutingRequestedPriority::Domestic => "domestic",
            PaymentMethodOptionsCardPresentRoutingRequestedPriority::International => "international",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn default() -> Self {
        Self::Domestic
    }
}
