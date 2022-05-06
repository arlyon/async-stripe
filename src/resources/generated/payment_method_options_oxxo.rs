// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_oxxo".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsOxxo {

    /// The number of calendar days before an OXXO invoice expires.
    ///
    /// For example, if you create an OXXO invoice on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    pub expires_after_days: u32,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentMethodOptionsOxxoSetupFutureUsage>,
}

/// An enum representing the possible values of an `PaymentMethodOptionsOxxo`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl PaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsOxxoSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsOxxoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
