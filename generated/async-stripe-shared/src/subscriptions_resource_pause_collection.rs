/// The Pause Collection settings determine how we will pause collection for this subscription and for how long the subscription.
/// should be paused.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePauseCollection {
    /// The payment collection behavior for this subscription while paused.
    pub behavior: stripe_shared::Behavior,
    /// The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourcePauseCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourcePauseCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourcePauseCollectionBuilder {
    behavior: Option<stripe_shared::Behavior>,
    resumes_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for SubscriptionsResourcePauseCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePauseCollection>,
        builder: SubscriptionsResourcePauseCollectionBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePauseCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourcePauseCollectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePauseCollectionBuilder {
        type Out = SubscriptionsResourcePauseCollection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "behavior" => Deserialize::begin(&mut self.behavior),
                "resumes_at" => Deserialize::begin(&mut self.resumes_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { behavior: None, resumes_at: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(behavior), Some(resumes_at)) = (self.behavior.take(), self.resumes_at) else {
                return None;
            };
            Some(Self::Out { behavior, resumes_at })
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

    impl ObjectDeser for SubscriptionsResourcePauseCollection {
        type Builder = SubscriptionsResourcePauseCollectionBuilder;
    }

    impl FromValueOpt for SubscriptionsResourcePauseCollection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourcePauseCollectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "behavior" => b.behavior = FromValueOpt::from_value(v),
                    "resumes_at" => b.resumes_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The payment collection behavior for this subscription while paused.
pub type SubscriptionsResourcePauseCollectionBehavior = stripe_shared::Behavior;
