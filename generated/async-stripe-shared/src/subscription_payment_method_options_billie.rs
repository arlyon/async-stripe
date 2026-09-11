#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPaymentMethodOptionsBillie {}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsBillie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsBillie").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPaymentMethodOptionsBillieBuilder {}

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

    impl Deserialize for SubscriptionPaymentMethodOptionsBillie {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPaymentMethodOptionsBillie>,
        builder: SubscriptionPaymentMethodOptionsBillieBuilder,
    }

    impl Visitor for Place<SubscriptionPaymentMethodOptionsBillie> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPaymentMethodOptionsBillieBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionPaymentMethodOptionsBillieBuilder {
        type Out = SubscriptionPaymentMethodOptionsBillie;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {}
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let () = () else {
                return None;
            };
            Some(Self::Out {})
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

    impl ObjectDeser for SubscriptionPaymentMethodOptionsBillie {
        type Builder = SubscriptionPaymentMethodOptionsBillieBuilder;
    }

    impl FromValueOpt for SubscriptionPaymentMethodOptionsBillie {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionPaymentMethodOptionsBillieBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
