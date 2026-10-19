#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    /// Controls whether this PaymentIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    /// To [confirm](https://docs.stripe.com/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
    pub allow_redirects: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// Automatically calculates compatible payment methods
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAutomaticPaymentMethodsPaymentIntent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder {
    allow_redirects: Option<Option<stripe_shared::MandatePixAmountIncludesIof>>,
    enabled: Option<bool>,
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

    impl Deserialize for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAutomaticPaymentMethodsPaymentIntent>,
        builder: PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder,
    }

    impl Visitor for Place<PaymentFlowsAutomaticPaymentMethodsPaymentIntent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder {
        type Out = PaymentFlowsAutomaticPaymentMethodsPaymentIntent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_redirects" => Deserialize::begin(&mut self.allow_redirects),
                "enabled" => Deserialize::begin(&mut self.enabled),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { allow_redirects: Some(None), enabled: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(allow_redirects), Some(enabled)) =
                (self.allow_redirects.take(), self.enabled)
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
        type Builder = PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder;
    }

    impl FromValueOpt for PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAutomaticPaymentMethodsPaymentIntentBuilder::deser_default();
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
/// Controls whether this PaymentIntent will accept redirect-based payment methods.
///
/// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
/// To [confirm](https://docs.stripe.com/api/payment_intents/confirm) this PaymentIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the payment.
pub type PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects =
    stripe_shared::MandatePixAmountIncludesIof;
