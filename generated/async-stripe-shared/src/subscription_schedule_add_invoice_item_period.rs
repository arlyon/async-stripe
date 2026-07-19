#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleAddInvoiceItemPeriod {
    pub end: stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd,
    pub start: stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionScheduleAddInvoiceItemPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionScheduleAddInvoiceItemPeriod").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionScheduleAddInvoiceItemPeriodBuilder {
    end: Option<stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd>,
    start: Option<stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart>,
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

    impl Deserialize for SubscriptionScheduleAddInvoiceItemPeriod {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleAddInvoiceItemPeriod>,
        builder: SubscriptionScheduleAddInvoiceItemPeriodBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleAddInvoiceItemPeriod> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionScheduleAddInvoiceItemPeriodBuilder {
                    end: Deserialize::default(),
                    start: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end" => Deserialize::begin(&mut self.builder.end),
                "start" => Deserialize::begin(&mut self.builder.start),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(end), Some(start)) = (self.builder.end.take(), self.builder.start.take())
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionScheduleAddInvoiceItemPeriod { end, start });
            Ok(())
        }
    }
};
