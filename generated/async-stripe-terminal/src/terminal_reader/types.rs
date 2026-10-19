/// A Reader represents a physical device for accepting payment details.
///
/// Related guide: [Connecting to a reader](https://docs.stripe.com/terminal/payments/connect-reader)
///
/// For more details see <<https://stripe.com/docs/api/terminal/readers/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReader {
    /// The most recent action performed by the reader.
    pub action: Option<stripe_terminal::TerminalReaderReaderResourceReaderAction>,
    /// The current software version of the reader.
    pub device_sw_version: Option<String>,
    /// Device type of the reader.
    pub device_type: stripe_shared::DeviceType,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalReaderId,
    /// The local IP address of the reader.
    pub ip_address: Option<String>,
    /// Custom label given to the reader for easier identification.
    pub label: String,
    /// The last time this reader reported to Stripe backend.
    /// Timestamp is measured in milliseconds since the Unix epoch.
    /// Unlike most other Stripe timestamp fields which use seconds, this field uses milliseconds.
    pub last_seen_at: Option<stripe_types::Timestamp>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The location identifier of the reader.
    pub location: Option<stripe_types::Expandable<stripe_terminal::TerminalLocation>>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Serial number of the reader.
    pub serial_number: String,
    /// The networking status of the reader.
    /// We do not recommend using this field in flows that may block taking payments.
    pub status: Option<stripe_shared::TerminalReaderStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReader").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderBuilder {
    action: Option<Option<stripe_terminal::TerminalReaderReaderResourceReaderAction>>,
    device_sw_version: Option<Option<String>>,
    device_type: Option<stripe_shared::DeviceType>,
    id: Option<stripe_terminal::TerminalReaderId>,
    ip_address: Option<Option<String>>,
    label: Option<String>,
    last_seen_at: Option<Option<stripe_types::Timestamp>>,
    livemode: Option<bool>,
    location: Option<Option<stripe_types::Expandable<stripe_terminal::TerminalLocation>>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    serial_number: Option<String>,
    status: Option<Option<stripe_shared::TerminalReaderStatus>>,
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

    impl Deserialize for TerminalReader {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReader>,
        builder: TerminalReaderBuilder,
    }

    impl Visitor for Place<TerminalReader> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderBuilder {
        type Out = TerminalReader;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.action),
                "device_sw_version" => Deserialize::begin(&mut self.device_sw_version),
                "device_type" => Deserialize::begin(&mut self.device_type),
                "id" => Deserialize::begin(&mut self.id),
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "label" => Deserialize::begin(&mut self.label),
                "last_seen_at" => Deserialize::begin(&mut self.last_seen_at),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "location" => Deserialize::begin(&mut self.location),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "serial_number" => Deserialize::begin(&mut self.serial_number),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                action: Some(None),
                device_sw_version: Some(None),
                device_type: None,
                id: None,
                ip_address: Some(None),
                label: None,
                last_seen_at: Some(None),
                livemode: None,
                location: Some(None),
                metadata: None,
                serial_number: None,
                status: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(action),
                Some(device_sw_version),
                Some(device_type),
                Some(id),
                Some(ip_address),
                Some(label),
                Some(last_seen_at),
                Some(livemode),
                Some(location),
                Some(metadata),
                Some(serial_number),
                Some(status),
            ) = (
                self.action.take(),
                self.device_sw_version.take(),
                self.device_type.take(),
                self.id.take(),
                self.ip_address.take(),
                self.label.take(),
                self.last_seen_at,
                self.livemode,
                self.location.take(),
                self.metadata.take(),
                self.serial_number.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                action,
                device_sw_version,
                device_type,
                id,
                ip_address,
                label,
                last_seen_at,
                livemode,
                location,
                metadata,
                serial_number,
                status,
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

    impl ObjectDeser for TerminalReader {
        type Builder = TerminalReaderBuilder;
    }

    impl FromValueOpt for TerminalReader {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "action" => b.action = FromValueOpt::from_value(v),
                    "device_sw_version" => b.device_sw_version = FromValueOpt::from_value(v),
                    "device_type" => b.device_type = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "label" => b.label = FromValueOpt::from_value(v),
                    "last_seen_at" => b.last_seen_at = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "location" => b.location = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "serial_number" => b.serial_number = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReader {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalReader", 13)?;
        s.serialize_field("action", &self.action)?;
        s.serialize_field("device_sw_version", &self.device_sw_version)?;
        s.serialize_field("device_type", &self.device_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ip_address", &self.ip_address)?;
        s.serialize_field("label", &self.label)?;
        s.serialize_field("last_seen_at", &self.last_seen_at)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("location", &self.location)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("serial_number", &self.serial_number)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "terminal.reader")?;
        s.end()
    }
}
impl stripe_types::Object for TerminalReader {
    type Id = stripe_terminal::TerminalReaderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TerminalReaderId);
/// Device type of the reader.
pub type TerminalReaderDeviceType = stripe_shared::DeviceType;
/// The networking status of the reader.
/// We do not recommend using this field in flows that may block taking payments.
pub type TerminalReaderStatus = stripe_shared::TerminalReaderStatus;
