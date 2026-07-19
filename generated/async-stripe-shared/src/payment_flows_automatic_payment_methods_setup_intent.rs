#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://docs.stripe.com/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    pub allow_redirects: Option<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods
    pub enabled: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAutomaticPaymentMethodsSetupIntent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder {
    allow_redirects: Option<Option<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>>,
    enabled: Option<Option<bool>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for PaymentFlowsAutomaticPaymentMethodsSetupIntent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAutomaticPaymentMethodsSetupIntent>,
        builder: PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder,
    }

    impl Visitor for Place<PaymentFlowsAutomaticPaymentMethodsSetupIntent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder {
                    allow_redirects: Deserialize::default(),
                    enabled: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_redirects" => Deserialize::begin(&mut self.builder.allow_redirects),
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(allow_redirects), Some(enabled)) =
                (self.builder.allow_redirects.take(), self.builder.enabled)
            else {
                return Ok(());
            };
            *self.out =
                Some(PaymentFlowsAutomaticPaymentMethodsSetupIntent { allow_redirects, enabled });
            Ok(())
        }
    }
};
/// Controls whether this SetupIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
/// To [confirm](https://docs.stripe.com/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    Always,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match self {
            Always => "always",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
