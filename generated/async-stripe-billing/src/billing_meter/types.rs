/// Meters specify how to aggregate meter events over a billing period.
/// Meter events represent the actions that customers take in your system.
/// Meters attach to prices and form the basis of the bill.
///
/// Related guide: [Usage based billing](https://docs.stripe.com/billing/subscriptions/usage-based)
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeter {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub customer_mapping: stripe_billing::BillingMeterResourceCustomerMappingSettings,
    pub default_aggregation: stripe_billing::BillingMeterResourceAggregationSettings,
    /// The meter's name.
    pub display_name: String,
    /// The name of the meter event to record usage for.
    /// Corresponds with the `event_name` field on meter events.
    pub event_name: String,
    /// The time window which meter events have been pre-aggregated for, if any.
    pub event_time_window: Option<stripe_shared::BillingMeterEventTimeWindow>,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingMeterId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The meter's status.
    pub status: stripe_shared::BillingMeterStatus,
    pub status_transitions: stripe_billing::BillingMeterResourceBillingMeterStatusTransitions,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    pub value_settings: stripe_billing::BillingMeterResourceBillingMeterValue,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeter").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingMeterBuilder {
    created: Option<stripe_types::Timestamp>,
    customer_mapping: Option<stripe_billing::BillingMeterResourceCustomerMappingSettings>,
    default_aggregation: Option<stripe_billing::BillingMeterResourceAggregationSettings>,
    display_name: Option<String>,
    event_name: Option<String>,
    event_time_window: Option<Option<stripe_shared::BillingMeterEventTimeWindow>>,
    id: Option<stripe_billing::BillingMeterId>,
    livemode: Option<bool>,
    status: Option<stripe_shared::BillingMeterStatus>,
    status_transitions: Option<stripe_billing::BillingMeterResourceBillingMeterStatusTransitions>,
    updated: Option<stripe_types::Timestamp>,
    value_settings: Option<stripe_billing::BillingMeterResourceBillingMeterValue>,
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

    impl Deserialize for BillingMeter {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeter>,
        builder: BillingMeterBuilder,
    }

    impl Visitor for Place<BillingMeter> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterBuilder {
        type Out = BillingMeter;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "customer_mapping" => Deserialize::begin(&mut self.customer_mapping),
                "default_aggregation" => Deserialize::begin(&mut self.default_aggregation),
                "display_name" => Deserialize::begin(&mut self.display_name),
                "event_name" => Deserialize::begin(&mut self.event_name),
                "event_time_window" => Deserialize::begin(&mut self.event_time_window),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),
                "updated" => Deserialize::begin(&mut self.updated),
                "value_settings" => Deserialize::begin(&mut self.value_settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: None,
                customer_mapping: None,
                default_aggregation: None,
                display_name: None,
                event_name: None,
                event_time_window: Some(None),
                id: None,
                livemode: None,
                status: None,
                status_transitions: None,
                updated: None,
                value_settings: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(customer_mapping),
                Some(default_aggregation),
                Some(display_name),
                Some(event_name),
                Some(event_time_window),
                Some(id),
                Some(livemode),
                Some(status),
                Some(status_transitions),
                Some(updated),
                Some(value_settings),
            ) = (
                self.created,
                self.customer_mapping.take(),
                self.default_aggregation.take(),
                self.display_name.take(),
                self.event_name.take(),
                self.event_time_window.take(),
                self.id.take(),
                self.livemode,
                self.status.take(),
                self.status_transitions,
                self.updated,
                self.value_settings.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                customer_mapping,
                default_aggregation,
                display_name,
                event_name,
                event_time_window,
                id,
                livemode,
                status,
                status_transitions,
                updated,
                value_settings,
            })
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

    impl ObjectDeser for BillingMeter {
        type Builder = BillingMeterBuilder;
    }

    impl FromValueOpt for BillingMeter {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer_mapping" => b.customer_mapping = FromValueOpt::from_value(v),
                    "default_aggregation" => b.default_aggregation = FromValueOpt::from_value(v),
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "event_name" => b.event_name = FromValueOpt::from_value(v),
                    "event_time_window" => b.event_time_window = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_transitions" => b.status_transitions = FromValueOpt::from_value(v),
                    "updated" => b.updated = FromValueOpt::from_value(v),
                    "value_settings" => b.value_settings = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeter {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingMeter", 13)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer_mapping", &self.customer_mapping)?;
        s.serialize_field("default_aggregation", &self.default_aggregation)?;
        s.serialize_field("display_name", &self.display_name)?;
        s.serialize_field("event_name", &self.event_name)?;
        s.serialize_field("event_time_window", &self.event_time_window)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("updated", &self.updated)?;
        s.serialize_field("value_settings", &self.value_settings)?;

        s.serialize_field("object", "billing.meter")?;
        s.end()
    }
}
impl stripe_types::Object for BillingMeter {
    type Id = stripe_billing::BillingMeterId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingMeterId);
/// The time window which meter events have been pre-aggregated for, if any.
pub type BillingMeterEventTimeWindow = stripe_shared::BillingMeterEventTimeWindow;
/// The meter's status.
pub type BillingMeterStatus = stripe_shared::BillingMeterStatus;
