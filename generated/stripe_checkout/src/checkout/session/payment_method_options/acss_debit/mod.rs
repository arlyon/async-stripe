#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AcssDebit {
    /// Currency supported by the bank account.
    ///
    /// Returned when the Session is in `setup` mode.
#[serde(skip_serializing_if = "Option::is_none")]
pub currency: Option<AcssDebitCurrency>,
#[serde(skip_serializing_if = "Option::is_none")]
pub mandate_options: Option<stripe_checkout::checkout::session::payment_method_options::acss_debit::mandate_options::MandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[serde(skip_serializing_if = "Option::is_none")]
pub setup_future_usage: Option<AcssDebitSetupFutureUsage>,
    /// Bank account verification method.
#[serde(skip_serializing_if = "Option::is_none")]
pub verification_method: Option<AcssDebitVerificationMethod>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Currency supported by the bank account.
///
/// Returned when the Session is in `setup` mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitCurrency {
    Cad,
    Usd,
}

impl AcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl std::str::FromStr for AcssDebitCurrency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cad" => Ok(Self::Cad),
            "usd" => Ok(Self::Usd),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitCurrency"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitCurrency::from_str(s)?);
        Ok(())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl AcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for AcssDebitSetupFutureUsage {
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

impl AsRef<str> for AcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitSetupFutureUsage"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitSetupFutureUsage::from_str(s)?);
        Ok(())
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl AcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for AcssDebitVerificationMethod {
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

impl AsRef<str> for AcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitVerificationMethod"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitVerificationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitVerificationMethod::from_str(s)?);
        Ok(())
    }
}
pub mod mandate_options;
pub use mandate_options::MandateOptions;
