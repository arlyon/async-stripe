#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AfterpayClearpay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<AfterpayClearpayCaptureMethod>,
    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    pub reference: Option<String>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<AfterpayClearpaySetupFutureUsage>,
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AfterpayClearpayCaptureMethod {
    Manual,
}

impl AfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for AfterpayClearpayCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AfterpayClearpayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AfterpayClearpayCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AfterpayClearpayCaptureMethod")
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AfterpayClearpaySetupFutureUsage {
    None,
}

impl AfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for AfterpayClearpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AfterpayClearpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AfterpayClearpaySetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AfterpayClearpaySetupFutureUsage")
        })
    }
}
