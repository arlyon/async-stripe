#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    /// Currency supported by the bank account. Returned when the Session is in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CheckoutAcssDebitPaymentMethodOptionsCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_checkout::CheckoutAcssDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
    ///
    /// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>,
}
/// Currency supported by the bank account. Returned when the Session is in `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsCurrency {
    Cad,
    Usd,
}
impl CheckoutAcssDebitPaymentMethodOptionsCurrency {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitPaymentMethodOptionsCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitPaymentMethodOptionsCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutAcssDebitPaymentMethodOptionsCurrency",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.
///
/// When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage",
            )
        })
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod",
            )
        })
    }
}
