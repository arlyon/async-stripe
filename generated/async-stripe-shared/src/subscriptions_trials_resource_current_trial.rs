/// The current trial attached to the subscription item.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsTrialsResourceCurrentTrial {
    pub end_date: stripe_types::Timestamp,
    pub start_date: stripe_types::Timestamp,
    pub trial_offer: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsTrialsResourceCurrentTrial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsTrialsResourceCurrentTrial").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsTrialsResourceCurrentTrialBuilder {
    end_date: Option<stripe_types::Timestamp>,
    start_date: Option<stripe_types::Timestamp>,
    trial_offer: Option<String>,
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

    impl Deserialize for SubscriptionsTrialsResourceCurrentTrial {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsTrialsResourceCurrentTrial>,
        builder: SubscriptionsTrialsResourceCurrentTrialBuilder,
    }

    impl Visitor for Place<SubscriptionsTrialsResourceCurrentTrial> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsTrialsResourceCurrentTrialBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsTrialsResourceCurrentTrialBuilder {
        type Out = SubscriptionsTrialsResourceCurrentTrial;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end_date" => Deserialize::begin(&mut self.end_date),
                "start_date" => Deserialize::begin(&mut self.start_date),
                "trial_offer" => Deserialize::begin(&mut self.trial_offer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end_date: None, start_date: None, trial_offer: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(end_date), Some(start_date), Some(trial_offer)) =
                (self.end_date, self.start_date, self.trial_offer.take())
            else {
                return None;
            };
            Some(Self::Out { end_date, start_date, trial_offer })
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

    impl ObjectDeser for SubscriptionsTrialsResourceCurrentTrial {
        type Builder = SubscriptionsTrialsResourceCurrentTrialBuilder;
    }

    impl FromValueOpt for SubscriptionsTrialsResourceCurrentTrial {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsTrialsResourceCurrentTrialBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    "trial_offer" => b.trial_offer = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
