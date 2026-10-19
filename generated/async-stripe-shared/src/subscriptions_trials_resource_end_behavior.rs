/// Defines how a subscription behaves when a free trial ends.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsTrialsResourceEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: stripe_shared::MissingPaymentMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsTrialsResourceEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsTrialsResourceEndBehavior").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsTrialsResourceEndBehaviorBuilder {
    missing_payment_method: Option<stripe_shared::MissingPaymentMethod>,
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

    impl Deserialize for SubscriptionsTrialsResourceEndBehavior {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsTrialsResourceEndBehavior>,
        builder: SubscriptionsTrialsResourceEndBehaviorBuilder,
    }

    impl Visitor for Place<SubscriptionsTrialsResourceEndBehavior> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsTrialsResourceEndBehaviorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsTrialsResourceEndBehaviorBuilder {
        type Out = SubscriptionsTrialsResourceEndBehavior;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "missing_payment_method" => Deserialize::begin(&mut self.missing_payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { missing_payment_method: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(missing_payment_method),) = (self.missing_payment_method.take(),) else {
                return None;
            };
            Some(Self::Out { missing_payment_method })
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

    impl ObjectDeser for SubscriptionsTrialsResourceEndBehavior {
        type Builder = SubscriptionsTrialsResourceEndBehaviorBuilder;
    }

    impl FromValueOpt for SubscriptionsTrialsResourceEndBehavior {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsTrialsResourceEndBehaviorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "missing_payment_method" => {
                        b.missing_payment_method = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
pub type SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod =
    stripe_shared::MissingPaymentMethod;
