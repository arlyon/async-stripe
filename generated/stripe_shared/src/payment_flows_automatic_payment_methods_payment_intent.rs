#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    /// Controls whether this PaymentIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods
    pub enabled: bool,
}
/// Controls whether this PaymentIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
/// To [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    Always,
    Never,
}
impl PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    pub fn as_str(self) -> &'static str {
        use PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects",
            )
        })
    }
}
