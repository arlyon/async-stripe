#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    pub allow_redirects: Option<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>,
    /// Automatically calculates compatible payment methods
    pub enabled: Option<bool>,
}
#[doc(hidden)]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder {
    allow_redirects: Option<Option<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>>,
    enabled: Option<Option<bool>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder {
        type Out = PaymentFlowsAutomaticPaymentMethodsSetupIntent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_redirects" => Deserialize::begin(&mut self.allow_redirects),
                "enabled" => Deserialize::begin(&mut self.enabled),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { allow_redirects: Deserialize::default(), enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(allow_redirects), Some(enabled)) = (self.allow_redirects, self.enabled)
            else {
                return None;
            };
            Some(Self::Out { allow_redirects, enabled })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentFlowsAutomaticPaymentMethodsSetupIntent {
        type Builder = PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder;
    }

    impl FromValueOpt for PaymentFlowsAutomaticPaymentMethodsSetupIntent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAutomaticPaymentMethodsSetupIntentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allow_redirects" => b.allow_redirects = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls whether this SetupIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects",
            )
        })
    }
}
