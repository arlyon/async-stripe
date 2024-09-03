/// A billing meter is a resource that allows you to track usage of a particular event.
/// For example, you might create a billing meter to track the number of API calls made by a particular user.
/// You can then attach the billing meter to a price and attach the price to a subscription to charge the user for the number of API calls they make.
#[derive(Clone, Debug)]
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
    /// The time window to pre-aggregate meter events for, if any.
    pub event_time_window: Option<stripe_billing::BillingMeterEventTimeWindow>,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingMeterId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The meter's status.
    pub status: stripe_billing::BillingMeterStatus,
    pub status_transitions: stripe_billing::BillingMeterResourceBillingMeterStatusTransitions,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    pub value_settings: stripe_billing::BillingMeterResourceBillingMeterValue,
}
#[doc(hidden)]
pub struct BillingMeterBuilder {
    created: Option<stripe_types::Timestamp>,
    customer_mapping: Option<stripe_billing::BillingMeterResourceCustomerMappingSettings>,
    default_aggregation: Option<stripe_billing::BillingMeterResourceAggregationSettings>,
    display_name: Option<String>,
    event_name: Option<String>,
    event_time_window: Option<Option<stripe_billing::BillingMeterEventTimeWindow>>,
    id: Option<stripe_billing::BillingMeterId>,
    livemode: Option<bool>,
    status: Option<stripe_billing::BillingMeterStatus>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                created: Deserialize::default(),
                customer_mapping: Deserialize::default(),
                default_aggregation: Deserialize::default(),
                display_name: Deserialize::default(),
                event_name: Deserialize::default(),
                event_time_window: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                updated: Deserialize::default(),
                value_settings: Deserialize::default(),
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
                self.default_aggregation,
                self.display_name.take(),
                self.event_name.take(),
                self.event_time_window,
                self.id.take(),
                self.livemode,
                self.status,
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

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingMeterEventTimeWindow {
    Day,
    Hour,
}
impl BillingMeterEventTimeWindow {
    pub fn as_str(self) -> &'static str {
        use BillingMeterEventTimeWindow::*;
        match self {
            Day => "day",
            Hour => "hour",
        }
    }
}

impl std::str::FromStr for BillingMeterEventTimeWindow {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventTimeWindow::*;
        match s {
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BillingMeterEventTimeWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingMeterEventTimeWindow {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterEventTimeWindow> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterEventTimeWindow::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterEventTimeWindow);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventTimeWindow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BillingMeterEventTimeWindow"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingMeterStatus {
    Active,
    Inactive,
}
impl BillingMeterStatus {
    pub fn as_str(self) -> &'static str {
        use BillingMeterStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for BillingMeterStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingMeterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingMeterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BillingMeterStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingMeterStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BillingMeterStatus"))
    }
}
