#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentTypeSpecificPaymentMethodOptionsClient {
    /// Controls when the funds will be captured from the customer's account.
#[serde(skip_serializing_if = "Option::is_none")]
pub capture_method: Option<PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod>,
#[serde(skip_serializing_if = "Option::is_none")]
pub installments: Option<stripe_core::payment_intent_type_specific_payment_method_options_client::installments::Installments>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[serde(skip_serializing_if = "Option::is_none")]
pub setup_future_usage: Option<PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage>,
    /// Bank account verification method.
#[serde(skip_serializing_if = "Option::is_none")]
pub verification_method: Option<PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentTypeSpecificPaymentMethodOptionsClient {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    Manual,
}

impl PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod installments;
pub use installments::Installments;
