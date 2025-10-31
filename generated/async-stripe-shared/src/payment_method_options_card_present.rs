#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardPresent {
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity).
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    pub request_incremental_authorization_support: Option<bool>,
    pub routing: Option<stripe_shared::PaymentMethodOptionsCardPresentRouting>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCardPresentBuilder {
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
                request_extended_authorization: Deserialize::default(),
                request_incremental_authorization_support: Deserialize::default(),
                routing: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(request_extended_authorization),
                Some(request_incremental_authorization_support),
                Some(routing),
            ) = (
                self.request_extended_authorization,
                self.request_incremental_authorization_support,
                self.routing,
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            *self.out = self.builder.take_out();
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
