#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    /// Defaults to 86400 seconds.
    pub expires_after_seconds: Option<i64>,
    pub mandate_options: Option<stripe_shared::SubscriptionPaymentMethodOptionsMandateOptionsPix>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsPix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPaymentMethodOptionsPixBuilder {
    expires_after_seconds: Option<Option<i64>>,
    mandate_options:
        Option<Option<stripe_shared::SubscriptionPaymentMethodOptionsMandateOptionsPix>>,
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

    impl Deserialize for SubscriptionPaymentMethodOptionsPix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPaymentMethodOptionsPix>,
        builder: SubscriptionPaymentMethodOptionsPixBuilder,
    }

    impl Visitor for Place<SubscriptionPaymentMethodOptionsPix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPaymentMethodOptionsPixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionPaymentMethodOptionsPixBuilder {
        type Out = SubscriptionPaymentMethodOptionsPix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_after_seconds" => Deserialize::begin(&mut self.expires_after_seconds),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { expires_after_seconds: Some(None), mandate_options: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(expires_after_seconds), Some(mandate_options)) =
                (self.expires_after_seconds, self.mandate_options.take())
            else {
                return None;
            };
            Some(Self::Out { expires_after_seconds, mandate_options })
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

    impl ObjectDeser for SubscriptionPaymentMethodOptionsPix {
        type Builder = SubscriptionPaymentMethodOptionsPixBuilder;
    }

    impl FromValueOpt for SubscriptionPaymentMethodOptionsPix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionPaymentMethodOptionsPixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expires_after_seconds" => {
                        b.expires_after_seconds = FromValueOpt::from_value(v)
                    }
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
