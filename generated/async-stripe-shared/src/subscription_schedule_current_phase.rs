#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleCurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct SubscriptionScheduleCurrentPhaseBuilder {
    end_date: Option<stripe_types::Timestamp>,
    start_date: Option<stripe_types::Timestamp>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionScheduleCurrentPhase {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleCurrentPhase>,
        builder: SubscriptionScheduleCurrentPhaseBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleCurrentPhase> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionScheduleCurrentPhaseBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionScheduleCurrentPhaseBuilder {
        type Out = SubscriptionScheduleCurrentPhase;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end_date" => Deserialize::begin(&mut self.end_date),
                "start_date" => Deserialize::begin(&mut self.start_date),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end_date: Deserialize::default(), start_date: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { end_date: self.end_date?, start_date: self.start_date? })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionScheduleCurrentPhase {
        type Builder = SubscriptionScheduleCurrentPhaseBuilder;
    }

    impl FromValueOpt for SubscriptionScheduleCurrentPhase {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionScheduleCurrentPhaseBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end_date" => b.end_date = Some(FromValueOpt::from_value(v)?),
                    "start_date" => b.start_date = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
