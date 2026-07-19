/// Meter events represent actions that customers take in your system.
/// You can use meter events to bill a customer based on their usage.
/// Meter events are associated with billing meters, which define both the contents of the event’s payload and how to aggregate those events.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterEvent {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name of the meter event. Corresponds with the `event_name` field on a meter.
    pub event_name: String,
    /// A unique identifier for the event.
    pub identifier: String,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The payload of the event.
    /// This contains the fields corresponding to a meter's `customer_mapping.event_payload_key` (default is `stripe_customer_id`) and `value_settings.event_payload_key` (default is `value`).
    /// Read more about the [payload](https://docs.stripe.com/billing/subscriptions/usage-based/meters/configure#meter-configuration-attributes).
    pub payload: std::collections::HashMap<String, String>,
    /// The timestamp passed in when creating the event. Measured in seconds since the Unix epoch.
    pub timestamp: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterEvent").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingMeterEventBuilder {
                    created: Deserialize::default(),
                    event_name: Deserialize::default(),
                    identifier: Deserialize::default(),
                    livemode: Deserialize::default(),
                    payload: Deserialize::default(),
                    timestamp: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "event_name" => Deserialize::begin(&mut self.builder.event_name),
                "identifier" => Deserialize::begin(&mut self.builder.identifier),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "payload" => Deserialize::begin(&mut self.builder.payload),
                "timestamp" => Deserialize::begin(&mut self.builder.timestamp),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(event_name),
                Some(identifier),
                Some(livemode),
                Some(payload),
                Some(timestamp),
            ) = (
                self.builder.created,
                self.builder.event_name.take(),
                self.builder.identifier.take(),
                self.builder.livemode,
                self.builder.payload.take(),
                self.builder.timestamp,
            )
            else {
                return Ok(());
            };
            *self.out = Some(BillingMeterEvent {
                created,
                event_name,
                identifier,
                livemode,
                payload,
                timestamp,
            });
            Ok(())
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
