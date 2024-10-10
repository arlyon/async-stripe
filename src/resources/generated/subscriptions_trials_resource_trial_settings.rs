// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionsTrialsResourceTrialSettings".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsTrialsResourceTrialSettings {
    pub end_behavior: SubscriptionsTrialsResourceEndBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsTrialsResourceEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod,
}

/// An enum representing the possible values of an `SubscriptionsTrialsResourceEndBehavior`'s `missing_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::CreateInvoice => {
                "create_invoice"
            }
            SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::Pause => "pause",
        }
    }
}

impl AsRef<str> for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn default() -> Self {
        Self::Cancel
    }
}
