#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleAddInvoiceItemPeriod {
    pub end: stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd,
    pub start: stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart,
}
#[doc(hidden)]
pub struct SubscriptionScheduleAddInvoiceItemPeriodBuilder {
    end: Option<stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd>,
    start: Option<stripe_shared::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart>,
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
                builder: SubscriptionScheduleAddInvoiceItemPeriodBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionScheduleAddInvoiceItemPeriodBuilder {
        type Out = SubscriptionScheduleAddInvoiceItemPeriod;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end" => Deserialize::begin(&mut self.end),
                "start" => Deserialize::begin(&mut self.start),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end: Deserialize::default(), start: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(end), Some(start)) = (self.end, self.start) else {
                return None;
            };
            Some(Self::Out { end, start })
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

    impl ObjectDeser for SubscriptionScheduleAddInvoiceItemPeriod {
        type Builder = SubscriptionScheduleAddInvoiceItemPeriodBuilder;
    }

    impl FromValueOpt for SubscriptionScheduleAddInvoiceItemPeriod {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionScheduleAddInvoiceItemPeriodBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end" => b.end = FromValueOpt::from_value(v),
                    "start" => b.start = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
