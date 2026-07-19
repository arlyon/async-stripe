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
    pub device_type: stripe_terminal::TerminalReaderDeviceType,
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
    pub status: Option<stripe_terminal::TerminalReaderStatus>,
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
    device_type: Option<stripe_terminal::TerminalReaderDeviceType>,
    id: Option<stripe_terminal::TerminalReaderId>,
    ip_address: Option<Option<String>>,
    label: Option<String>,
    last_seen_at: Option<Option<stripe_types::Timestamp>>,
    livemode: Option<bool>,
    location: Option<Option<stripe_types::Expandable<stripe_terminal::TerminalLocation>>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    serial_number: Option<String>,
    status: Option<Option<stripe_terminal::TerminalReaderStatus>>,
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
                builder: TerminalReaderBuilder {
                    action: Deserialize::default(),
                    device_sw_version: Deserialize::default(),
                    device_type: Deserialize::default(),
                    id: Deserialize::default(),
                    ip_address: Deserialize::default(),
                    label: Deserialize::default(),
                    last_seen_at: Deserialize::default(),
                    livemode: Deserialize::default(),
                    location: Deserialize::default(),
                    metadata: Deserialize::default(),
                    serial_number: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.builder.action),
                "device_sw_version" => Deserialize::begin(&mut self.builder.device_sw_version),
                "device_type" => Deserialize::begin(&mut self.builder.device_type),
                "id" => Deserialize::begin(&mut self.builder.id),
                "ip_address" => Deserialize::begin(&mut self.builder.ip_address),
                "label" => Deserialize::begin(&mut self.builder.label),
                "last_seen_at" => Deserialize::begin(&mut self.builder.last_seen_at),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "location" => Deserialize::begin(&mut self.builder.location),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "serial_number" => Deserialize::begin(&mut self.builder.serial_number),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.action.take(),
                self.builder.device_sw_version.take(),
                self.builder.device_type.take(),
                self.builder.id.take(),
                self.builder.ip_address.take(),
                self.builder.label.take(),
                self.builder.last_seen_at,
                self.builder.livemode,
                self.builder.location.take(),
                self.builder.metadata.take(),
                self.builder.serial_number.take(),
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReader {
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
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    MobilePhoneReader,
    SimulatedStripeS700,
    SimulatedStripeS710,
    SimulatedWiseposE,
    StripeM2,
    StripeS700,
    StripeS710,
    VerifoneP400,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderDeviceType {
    pub fn as_str(&self) -> &str {
        use TerminalReaderDeviceType::*;
        match self {
            BbposChipper2x => "bbpos_chipper2x",
            BbposWisepad3 => "bbpos_wisepad3",
            BbposWiseposE => "bbpos_wisepos_e",
            MobilePhoneReader => "mobile_phone_reader",
            SimulatedStripeS700 => "simulated_stripe_s700",
            SimulatedStripeS710 => "simulated_stripe_s710",
            SimulatedWiseposE => "simulated_wisepos_e",
            StripeM2 => "stripe_m2",
            StripeS700 => "stripe_s700",
            StripeS710 => "stripe_s710",
            VerifoneP400 => "verifone_P400",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderDeviceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderDeviceType::*;
        match s {
            "bbpos_chipper2x" => Ok(BbposChipper2x),
            "bbpos_wisepad3" => Ok(BbposWisepad3),
            "bbpos_wisepos_e" => Ok(BbposWiseposE),
            "mobile_phone_reader" => Ok(MobilePhoneReader),
            "simulated_stripe_s700" => Ok(SimulatedStripeS700),
            "simulated_stripe_s710" => Ok(SimulatedStripeS710),
            "simulated_wisepos_e" => Ok(SimulatedWiseposE),
            "stripe_m2" => Ok(StripeM2),
            "stripe_s700" => Ok(StripeS700),
            "stripe_s710" => Ok(StripeS710),
            "verifone_P400" => Ok(VerifoneP400),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TerminalReaderDeviceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderDeviceType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TerminalReaderDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalReaderDeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TerminalReaderDeviceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderDeviceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderStatus {
    Offline,
    Online,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderStatus {
    pub fn as_str(&self) -> &str {
        use TerminalReaderStatus::*;
        match self {
            Offline => "offline",
            Online => "online",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderStatus::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TerminalReaderStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TerminalReaderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalReaderStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TerminalReaderStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
