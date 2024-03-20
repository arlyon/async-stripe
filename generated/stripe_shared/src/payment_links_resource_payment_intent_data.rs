#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourcePaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>,
    /// Extra information about the payment.
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    pub statement_descriptor: Option<String>,
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub transfer_group: Option<String>,
}
/// Indicates when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}
impl PaymentLinksResourcePaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourcePaymentIntentDataCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with the payment method collected during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}
impl PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourcePaymentIntentDataSetupFutureUsage",
            )
        })
    }
}
