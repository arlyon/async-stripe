#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceSubscriptionPresentmentDetails {
    /// Currency used for customer payments.
    pub presentment_currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceSubscriptionPresentmentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceSubscriptionPresentmentDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceSubscriptionPresentmentDetailsBuilder {
    presentment_currency: Option<stripe_types::Currency>,
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

    impl Deserialize for SubscriptionsResourceSubscriptionPresentmentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceSubscriptionPresentmentDetails>,
        builder: SubscriptionsResourceSubscriptionPresentmentDetailsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceSubscriptionPresentmentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceSubscriptionPresentmentDetailsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceSubscriptionPresentmentDetailsBuilder {
        type Out = SubscriptionsResourceSubscriptionPresentmentDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "presentment_currency" => Deserialize::begin(&mut self.presentment_currency),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { presentment_currency: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(presentment_currency),) = (self.presentment_currency.take(),) else {
                return None;
            };
            Some(Self::Out { presentment_currency })
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

    impl ObjectDeser for SubscriptionsResourceSubscriptionPresentmentDetails {
        type Builder = SubscriptionsResourceSubscriptionPresentmentDetailsBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceSubscriptionPresentmentDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceSubscriptionPresentmentDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "presentment_currency" => b.presentment_currency = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
