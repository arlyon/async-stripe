#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardPresent {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsCardPresentCaptureMethod>,
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://docs.stripe.com/terminal/features/extended-authorizations#authorization-validity).
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://docs.stripe.com/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    /// Check [incremental_authorization_supported](https://docs.stripe.com/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://docs.stripe.com/api/payment_intents/confirm) response to verify support.
    pub request_incremental_authorization_support: Option<bool>,
    pub routing: Option<stripe_shared::PaymentMethodOptionsCardPresentRouting>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsCardPresent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCardPresentBuilder {
    capture_method: Option<Option<stripe_shared::PaymentMethodOptionsCardPresentCaptureMethod>>,
    request_extended_authorization: Option<Option<bool>>,
    request_incremental_authorization_support: Option<Option<bool>>,
    routing: Option<Option<stripe_shared::PaymentMethodOptionsCardPresentRouting>>,
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

    impl Deserialize for PaymentMethodOptionsCardPresent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardPresent>,
        builder: PaymentMethodOptionsCardPresentBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardPresent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCardPresentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardPresentBuilder {
        type Out = PaymentMethodOptionsCardPresent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.capture_method),
                "request_extended_authorization" => {
                    Deserialize::begin(&mut self.request_extended_authorization)
                }
                "request_incremental_authorization_support" => {
                    Deserialize::begin(&mut self.request_incremental_authorization_support)
                }
                "routing" => Deserialize::begin(&mut self.routing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_method: Some(None),
                request_extended_authorization: Some(None),
                request_incremental_authorization_support: Some(None),
                routing: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(capture_method),
                Some(request_extended_authorization),
                Some(request_incremental_authorization_support),
                Some(routing),
            ) = (
                self.capture_method.take(),
                self.request_extended_authorization,
                self.request_incremental_authorization_support,
                self.routing.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                capture_method,
                request_extended_authorization,
                request_incremental_authorization_support,
                routing,
            })
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

    impl ObjectDeser for PaymentMethodOptionsCardPresent {
        type Builder = PaymentMethodOptionsCardPresentBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCardPresent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCardPresentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_method" => b.capture_method = FromValueOpt::from_value(v),
                    "request_extended_authorization" => {
                        b.request_extended_authorization = FromValueOpt::from_value(v)
                    }
                    "request_incremental_authorization_support" => {
                        b.request_incremental_authorization_support = FromValueOpt::from_value(v)
                    }
                    "routing" => b.routing = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls when the funds will be captured from the customer's account.
pub type PaymentMethodOptionsCardPresentCaptureMethod =
    stripe_shared::PaymentMethodOptionsCardPresentCaptureMethod;
