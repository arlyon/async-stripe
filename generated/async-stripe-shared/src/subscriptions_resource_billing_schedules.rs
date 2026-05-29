/// Sets the billing schedule for the subscription.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingSchedules {
    /// Specifies which subscription items the billing schedule applies to.
    pub applies_to: Option<Vec<stripe_shared::SubscriptionsResourceBillingSchedulesAppliesTo>>,
    pub bill_until: stripe_shared::SubscriptionsResourceBillingSchedulesBillUntil,
    /// Unique identifier for the billing schedule.
    pub key: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingSchedules {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingSchedules").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingSchedulesBuilder {
    applies_to: Option<Option<Vec<stripe_shared::SubscriptionsResourceBillingSchedulesAppliesTo>>>,
    bill_until: Option<stripe_shared::SubscriptionsResourceBillingSchedulesBillUntil>,
    key: Option<String>,
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

    impl Deserialize for SubscriptionsResourceBillingSchedules {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingSchedules>,
        builder: SubscriptionsResourceBillingSchedulesBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingSchedules> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingSchedulesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingSchedulesBuilder {
        type Out = SubscriptionsResourceBillingSchedules;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "applies_to" => Deserialize::begin(&mut self.applies_to),
                "bill_until" => Deserialize::begin(&mut self.bill_until),
                "key" => Deserialize::begin(&mut self.key),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { applies_to: Some(None), bill_until: None, key: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(applies_to), Some(bill_until), Some(key)) =
                (self.applies_to.take(), self.bill_until.take(), self.key.take())
            else {
                return None;
            };
            Some(Self::Out { applies_to, bill_until, key })
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

    impl ObjectDeser for SubscriptionsResourceBillingSchedules {
        type Builder = SubscriptionsResourceBillingSchedulesBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingSchedules {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceBillingSchedulesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "applies_to" => b.applies_to = FromValueOpt::from_value(v),
                    "bill_until" => b.bill_until = FromValueOpt::from_value(v),
                    "key" => b.key = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
