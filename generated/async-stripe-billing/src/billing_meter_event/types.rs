/// Meter events represent actions that customers take in your system.
/// You can use meter events to bill a customer based on their usage.
/// Meter events are associated with billing meters, which define both the contents of the event’s payload and how to aggregate those events.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterEvent {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name of the meter event. Corresponds with the `event_name` field on a meter.
    pub event_name: String,
    /// A unique identifier for the event.
    pub identifier: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The payload of the event.
    /// This contains the fields corresponding to a meter's `customer_mapping.event_payload_key` (default is `stripe_customer_id`) and `value_settings.event_payload_key` (default is `value`).
    /// Read more about the [payload](https://docs.stripe.com/billing/subscriptions/usage-based/meters/configure#meter-configuration-attributes).
    pub payload: std::collections::HashMap<String, String>,
    /// The timestamp passed in when creating the event. Measured in seconds since the Unix epoch.
    pub timestamp: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct BillingMeterEventBuilder {
    created: Option<stripe_types::Timestamp>,
    event_name: Option<String>,
    identifier: Option<String>,
    livemode: Option<bool>,
    payload: Option<std::collections::HashMap<String, String>>,
    timestamp: Option<stripe_types::Timestamp>,
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

    impl Deserialize for BillingMeterEvent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterEvent>,
        builder: BillingMeterEventBuilder,
    }

    impl Visitor for Place<BillingMeterEvent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterEventBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterEventBuilder {
        type Out = BillingMeterEvent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "event_name" => Deserialize::begin(&mut self.event_name),
                "identifier" => Deserialize::begin(&mut self.identifier),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "payload" => Deserialize::begin(&mut self.payload),
                "timestamp" => Deserialize::begin(&mut self.timestamp),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                event_name: Deserialize::default(),
                identifier: Deserialize::default(),
                livemode: Deserialize::default(),
                payload: Deserialize::default(),
                timestamp: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(event_name),
                Some(identifier),
                Some(livemode),
                Some(payload),
                Some(timestamp),
            ) = (
                self.created,
                self.event_name.take(),
                self.identifier.take(),
                self.livemode,
                self.payload.take(),
                self.timestamp,
            )
            else {
                return None;
            };
            Some(Self::Out { created, event_name, identifier, livemode, payload, timestamp })
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

    impl ObjectDeser for BillingMeterEvent {
        type Builder = BillingMeterEventBuilder;
    }

    impl FromValueOpt for BillingMeterEvent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterEventBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "event_name" => b.event_name = FromValueOpt::from_value(v),
                    "identifier" => b.identifier = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "payload" => b.payload = FromValueOpt::from_value(v),
                    "timestamp" => b.timestamp = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeterEvent {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingMeterEvent", 7)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("event_name", &self.event_name)?;
        s.serialize_field("identifier", &self.identifier)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("payload", &self.payload)?;
        s.serialize_field("timestamp", &self.timestamp)?;

        s.serialize_field("object", "billing.meter_event")?;
        s.end()
    }
}
