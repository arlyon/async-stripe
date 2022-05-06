// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_intent_payment_method_options_sepa_debit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentIntentPaymentMethodOptionsSepaDebit {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsSepaDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage::None => "none",
            PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage::OffSession => "off_session",
            PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
