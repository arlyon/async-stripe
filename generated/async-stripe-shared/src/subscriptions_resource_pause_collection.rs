/// The Pause Collection settings determine how we will pause collection for this subscription and for how long the subscription.
/// should be paused.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePauseCollection {
    /// The payment collection behavior for this subscription while paused.
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: SubscriptionsResourcePauseCollectionBehavior,
    /// The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct SubscriptionsResourcePauseCollectionBuilder {
    behavior: Option<SubscriptionsResourcePauseCollectionBehavior>,
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
    use miniserde::{make_place, Deserialize, Result};
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
            Self { behavior: Deserialize::default(), resumes_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(behavior), Some(resumes_at)) = (self.behavior, self.resumes_at) else {
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
            *self.out = self.builder.take_out();
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
/// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsResourcePauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}
impl SubscriptionsResourcePauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourcePauseCollectionBehavior::*;
        match self {
            KeepAsDraft => "keep_as_draft",
            MarkUncollectible => "mark_uncollectible",
            Void => "void",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePauseCollectionBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePauseCollectionBehavior::*;
        match s {
            "keep_as_draft" => Ok(KeepAsDraft),
            "mark_uncollectible" => Ok(MarkUncollectible),
            "void" => Ok(Void),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionsResourcePauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourcePauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourcePauseCollectionBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourcePauseCollectionBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionsResourcePauseCollectionBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourcePauseCollectionBehavior::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionsResourcePauseCollectionBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePauseCollectionBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionsResourcePauseCollectionBehavior",
            )
        })
    }
}
