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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor
    for crate::Place<PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
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
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize
    for PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor
    for crate::Place<PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
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
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "instant" => Ok(Self::Instant),
            "microdeposits" => Ok(Self::Microdeposits),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize
    for PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor
    for crate::Place<PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}
pub mod installments;
pub use installments::Installments;
