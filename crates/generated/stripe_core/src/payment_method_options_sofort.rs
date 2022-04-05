// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_sofort".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsSofort {
    /// Preferred language of the SOFORT authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<PaymentMethodOptionsSofortPreferredLanguage>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentMethodOptionsSofortSetupFutureUsage>,
}

/// An enum representing the possible values of an `PaymentMethodOptionsSofort`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl PaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsSofortPreferredLanguage::De => "de",
            PaymentMethodOptionsSofortPreferredLanguage::En => "en",
            PaymentMethodOptionsSofortPreferredLanguage::Es => "es",
            PaymentMethodOptionsSofortPreferredLanguage::Fr => "fr",
            PaymentMethodOptionsSofortPreferredLanguage::It => "it",
            PaymentMethodOptionsSofortPreferredLanguage::Nl => "nl",
            PaymentMethodOptionsSofortPreferredLanguage::Pl => "pl",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsSofortPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `PaymentMethodOptionsSofort`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl PaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsSofortSetupFutureUsage::None => "none",
            PaymentMethodOptionsSofortSetupFutureUsage::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsSofortSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
