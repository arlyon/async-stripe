#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    ///
    /// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods.
    pub enabled: Option<bool>,
}
/// Controls whether this SetupIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
///
/// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    Always,
    Never,
}

impl PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    pub fn as_str(self) -> &'static str {
        use PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects",
            )
        })
    }
}
