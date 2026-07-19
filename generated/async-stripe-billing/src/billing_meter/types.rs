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
    pub event_time_window: Option<stripe_billing::BillingMeterEventTimeWindow>,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingMeterId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The meter's status.
    pub status: stripe_billing::BillingMeterStatus,
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingMeterBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "customer_mapping" => Deserialize::begin(&mut self.builder.customer_mapping),
                "default_aggregation" => Deserialize::begin(&mut self.builder.default_aggregation),
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "event_name" => Deserialize::begin(&mut self.builder.event_name),
                "event_time_window" => Deserialize::begin(&mut self.builder.event_time_window),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_transitions" => Deserialize::begin(&mut self.builder.status_transitions),
                "updated" => Deserialize::begin(&mut self.builder.updated),
                "value_settings" => Deserialize::begin(&mut self.builder.value_settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.created,
                self.builder.customer_mapping.take(),
                self.builder.default_aggregation.take(),
                self.builder.display_name.take(),
                self.builder.event_name.take(),
                self.builder.event_time_window.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.status.take(),
                self.builder.status_transitions,
                self.builder.updated,
                self.builder.value_settings.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(BillingMeter {
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
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingMeterEventTimeWindow {
    Day,
    Hour,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingMeterEventTimeWindow {
    pub fn as_str(&self) -> &str {
        use BillingMeterEventTimeWindow::*;
        match self {
            Day => "day",
            Hour => "hour",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingMeterEventTimeWindow {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventTimeWindow::*;
        match s {
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingMeterEventTimeWindow"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingMeterEventTimeWindow)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BillingMeterEventTimeWindow {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingMeterEventTimeWindow> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterEventTimeWindow::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventTimeWindow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingMeterStatus {
    Active,
    Inactive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingMeterStatus {
    pub fn as_str(&self) -> &str {
        use BillingMeterStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingMeterStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "BillingMeterStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingMeterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingMeterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingMeterStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BillingMeterStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingMeterStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
