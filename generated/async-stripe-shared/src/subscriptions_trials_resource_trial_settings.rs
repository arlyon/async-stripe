/// Configures how this subscription behaves during the trial period.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsTrialsResourceTrialSettings {
    pub end_behavior: stripe_shared::SubscriptionsTrialsResourceEndBehavior,
}
#[doc(hidden)]
pub struct SubscriptionsTrialsResourceTrialSettingsBuilder {
    end_behavior: Option<stripe_shared::SubscriptionsTrialsResourceEndBehavior>,
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

    impl Deserialize for SubscriptionsTrialsResourceTrialSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsTrialsResourceTrialSettings>,
        builder: SubscriptionsTrialsResourceTrialSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionsTrialsResourceTrialSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsTrialsResourceTrialSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsTrialsResourceTrialSettingsBuilder {
        type Out = SubscriptionsTrialsResourceTrialSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end_behavior" => Deserialize::begin(&mut self.end_behavior),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end_behavior: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(end_behavior),) = (self.end_behavior.take(),) else {
                return None;
            };
            Some(Self::Out { end_behavior })
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

    impl ObjectDeser for SubscriptionsTrialsResourceTrialSettings {
        type Builder = SubscriptionsTrialsResourceTrialSettingsBuilder;
    }

    impl FromValueOpt for SubscriptionsTrialsResourceTrialSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsTrialsResourceTrialSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end_behavior" => b.end_behavior = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
