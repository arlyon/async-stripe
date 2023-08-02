#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomaticPaymentMethodsPaymentIntent {
    /// Controls whether this PaymentIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    ///
    /// To [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<AutomaticPaymentMethodsPaymentIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods.
    pub enabled: bool,
}
/// Controls whether this PaymentIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
///
/// To [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    Always,
    Never,
}

impl AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    pub fn as_str(self) -> &'static str {
        use AutomaticPaymentMethodsPaymentIntentAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AutomaticPaymentMethodsPaymentIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AutomaticPaymentMethodsPaymentIntentAllowRedirects",
            )
        })
    }
}
