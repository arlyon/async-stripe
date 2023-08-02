#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AutomaticPaymentMethodsSetupIntent {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    ///
    /// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<AutomaticPaymentMethodsSetupIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods.
    pub enabled: Option<bool>,
}
/// Controls whether this SetupIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
///
/// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AutomaticPaymentMethodsSetupIntentAllowRedirects {
    Always,
    Never,
}

impl AutomaticPaymentMethodsSetupIntentAllowRedirects {
    pub fn as_str(self) -> &'static str {
        use AutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
        }
    }
}

impl std::str::FromStr for AutomaticPaymentMethodsSetupIntentAllowRedirects {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AutomaticPaymentMethodsSetupIntentAllowRedirects",
            )
        })
    }
}
