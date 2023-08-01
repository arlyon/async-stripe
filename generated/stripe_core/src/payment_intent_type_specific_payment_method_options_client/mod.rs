#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    Manual,
    ManualPreferred,
}

impl PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod::*;
        match self {
            Manual => "manual",
            ManualPreferred => "manual_preferred",
        }
    }
}

impl std::str::FromStr for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            "manual_preferred" => Ok(ManualPreferred),
            _ => Err(()),
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
impl serde::Serialize for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
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
impl serde::Serialize for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage"))
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
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
impl serde::Serialize for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod"))
    }
}
pub mod installments;
pub use installments::Installments;
