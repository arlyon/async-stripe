#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalResourceScheduleUpdateAtPeriodEnd {
    /// List of conditions.
    /// When any condition is true, an update will be scheduled at the end of the current period.
    pub conditions: Vec<stripe_billing::PortalResourceScheduleUpdateAtPeriodEndCondition>,
}
#[doc(hidden)]
pub struct PortalResourceScheduleUpdateAtPeriodEndBuilder {
    conditions: Option<Vec<stripe_billing::PortalResourceScheduleUpdateAtPeriodEndCondition>>,
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

    impl Deserialize for PortalResourceScheduleUpdateAtPeriodEnd {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalResourceScheduleUpdateAtPeriodEnd>,
        builder: PortalResourceScheduleUpdateAtPeriodEndBuilder,
    }

    impl Visitor for Place<PortalResourceScheduleUpdateAtPeriodEnd> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalResourceScheduleUpdateAtPeriodEndBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalResourceScheduleUpdateAtPeriodEndBuilder {
        type Out = PortalResourceScheduleUpdateAtPeriodEnd;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "conditions" => Deserialize::begin(&mut self.conditions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { conditions: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(conditions),) = (self.conditions.take(),) else {
                return None;
            };
            Some(Self::Out { conditions })
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

    impl ObjectDeser for PortalResourceScheduleUpdateAtPeriodEnd {
        type Builder = PortalResourceScheduleUpdateAtPeriodEndBuilder;
    }

    impl FromValueOpt for PortalResourceScheduleUpdateAtPeriodEnd {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalResourceScheduleUpdateAtPeriodEndBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "conditions" => b.conditions = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
