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

    /// The method used to process this payment method offline.
    ///
    /// Only deferred is allowed.
    #[serde(rename = "type")]
    pub type_: Option<PaymentMethodDetailsCardPresentOfflineType>,
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresentOffline`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentOfflineType {
    Deferred,
}

impl PaymentMethodDetailsCardPresentOfflineType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentOfflineType::Deferred => "deferred",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentOfflineType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentOfflineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentOfflineType {
    fn default() -> Self {
        Self::Deferred
    }
}
