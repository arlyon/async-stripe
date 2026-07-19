/// A billing meter event summary represents an aggregated view of a customer's billing meter events within a specified timeframe.
/// It indicates how much.
/// usage was accrued by a customer for that period.
///
/// Note: Meters events are aggregated asynchronously so the meter event summaries provide an eventually consistent view of the reported usage.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterEventSummary {
    /// Aggregated value of all the events within `start_time` (inclusive) and `end_time` (inclusive).
    /// The aggregation strategy is defined on meter via `default_aggregation`.
    pub aggregated_value: f64,
    /// End timestamp for this event summary (exclusive). Must be aligned with minute boundaries.
    pub end_time: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingMeterEventSummaryId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The meter associated with this event summary.
    pub meter: String,
    /// Start timestamp for this event summary (inclusive). Must be aligned with minute boundaries.
    pub start_time: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEventSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterEventSummary").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingMeterEventSummaryBuilder {
                    aggregated_value: Deserialize::default(),
                    end_time: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    meter: Deserialize::default(),
                    start_time: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aggregated_value" => Deserialize::begin(&mut self.builder.aggregated_value),
                "end_time" => Deserialize::begin(&mut self.builder.end_time),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "meter" => Deserialize::begin(&mut self.builder.meter),
                "start_time" => Deserialize::begin(&mut self.builder.start_time),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(aggregated_value),
                Some(end_time),
                Some(id),
                Some(livemode),
                Some(meter),
                Some(start_time),
            ) = (
                self.builder.aggregated_value,
                self.builder.end_time,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.meter.take(),
                self.builder.start_time,
            )
            else {
                return Ok(());
            };
            *self.out = Some(BillingMeterEventSummary {
                aggregated_value,
                end_time,
                id,
                livemode,
                meter,
                start_time,
            });
            Ok(())
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
