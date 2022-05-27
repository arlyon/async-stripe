// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_intent_payment_method_options_acss_debit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentIntentPaymentMethodOptionsAcssDebit {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {

    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsAcssDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => "off_session",
            PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Combined => "combined",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Interval => "interval",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}

impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Business => "business",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn default() -> Self {
        Self::Business
    }
}
