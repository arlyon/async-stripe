/// A billing meter event summary represents an aggregated view of a customer's billing meter events within a specified timeframe.
/// It indicates how much.
/// usage was accrued by a customer for that period.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterEventSummary {
    /// Aggregated value of all the events within `start_time` (inclusive) and `end_time` (inclusive).
    /// The aggregation strategy is defined on meter via `default_aggregation`.
    pub aggregated_value: f64,
    /// End timestamp for this event summary (inclusive).
    pub end_time: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingMeterEventSummaryId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The meter associated with this event summary.
    pub meter: String,
    /// Start timestamp for this event summary (inclusive).
    pub start_time: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct BillingMeterEventSummaryBuilder {
    aggregated_value: Option<f64>,
    end_time: Option<stripe_types::Timestamp>,
    id: Option<stripe_billing::BillingMeterEventSummaryId>,
    livemode: Option<bool>,
    meter: Option<String>,
    start_time: Option<stripe_types::Timestamp>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for BillingMeterEventSummary {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterEventSummary>,
        builder: BillingMeterEventSummaryBuilder,
    }

    impl Visitor for Place<BillingMeterEventSummary> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterEventSummaryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterEventSummaryBuilder {
        type Out = BillingMeterEventSummary;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aggregated_value" => Deserialize::begin(&mut self.aggregated_value),
                "end_time" => Deserialize::begin(&mut self.end_time),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "meter" => Deserialize::begin(&mut self.meter),
                "start_time" => Deserialize::begin(&mut self.start_time),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                aggregated_value: Deserialize::default(),
                end_time: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                meter: Deserialize::default(),
                start_time: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(aggregated_value),
                Some(end_time),
                Some(id),
                Some(livemode),
                Some(meter),
                Some(start_time),
            ) = (
                self.aggregated_value,
                self.end_time,
                self.id.take(),
                self.livemode,
                self.meter.take(),
                self.start_time,
            )
            else {
                return None;
            };
            Some(Self::Out { aggregated_value, end_time, id, livemode, meter, start_time })
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

    impl ObjectDeser for BillingMeterEventSummary {
        type Builder = BillingMeterEventSummaryBuilder;
    }

    impl FromValueOpt for BillingMeterEventSummary {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterEventSummaryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "aggregated_value" => b.aggregated_value = FromValueOpt::from_value(v),
                    "end_time" => b.end_time = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "meter" => b.meter = FromValueOpt::from_value(v),
                    "start_time" => b.start_time = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeterEventSummary {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingMeterEventSummary", 7)?;
        s.serialize_field("aggregated_value", &self.aggregated_value)?;
        s.serialize_field("end_time", &self.end_time)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("meter", &self.meter)?;
        s.serialize_field("start_time", &self.start_time)?;

        s.serialize_field("object", "billing.meter_event_summary")?;
        s.end()
    }
}
impl stripe_types::Object for BillingMeterEventSummary {
    type Id = stripe_billing::BillingMeterEventSummaryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingMeterEventSummaryId);
