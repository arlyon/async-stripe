#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalResourceScheduleUpdateAtPeriodEnd {
    /// List of conditions.
    /// When any condition is true, an update will be scheduled at the end of the current period.
    pub conditions: Vec<stripe_billing::PortalResourceScheduleUpdateAtPeriodEndCondition>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalResourceScheduleUpdateAtPeriodEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalResourceScheduleUpdateAtPeriodEnd").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalResourceScheduleUpdateAtPeriodEndBuilder {
    conditions: Option<Vec<stripe_billing::PortalResourceScheduleUpdateAtPeriodEndCondition>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PortalResourceScheduleUpdateAtPeriodEndBuilder {
                    conditions: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "conditions" => Deserialize::begin(&mut self.builder.conditions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(conditions),) = (self.builder.conditions.take(),) else {
                return Ok(());
            };
            *self.out = Some(PortalResourceScheduleUpdateAtPeriodEnd { conditions });
            Ok(())
        }
    }
};
