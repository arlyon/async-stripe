// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_intent_payment_method_options_link".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentIntentPaymentMethodOptionsLink {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsLinkCaptureMethod>,

    /// Token used for persistent Link logins.
    pub persistent_token: Option<String>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage>,
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsLink`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsLinkCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsLink`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage::None => "none",
            PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
