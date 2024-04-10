/// A Reader represents a physical device for accepting payment details.
///
/// Related guide: [Connecting to a reader](https://stripe.com/docs/terminal/payments/connect-reader)
///
/// For more details see <<https://stripe.com/docs/api/terminal/readers/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReader {
    /// The most recent action performed by the reader.
    pub action: Option<stripe_terminal::TerminalReaderReaderResourceReaderAction>,
    /// The current software version of the reader.
    pub device_sw_version: Option<String>,
    /// Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`.
    pub device_type: stripe_terminal::TerminalReaderDeviceType,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalReaderId,
    /// The local IP address of the reader.
    pub ip_address: Option<String>,
    /// Custom label given to the reader for easier identification.
    pub label: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The location identifier of the reader.
    pub location: Option<stripe_types::Expandable<stripe_terminal::TerminalLocation>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Serial number of the reader.
    pub serial_number: String,
    /// The networking status of the reader.
    pub status: Option<stripe_terminal::TerminalReaderStatus>,
}
#[doc(hidden)]
pub struct TerminalReaderBuilder {
    action: Option<Option<stripe_terminal::TerminalReaderReaderResourceReaderAction>>,
    device_sw_version: Option<Option<String>>,
    device_type: Option<stripe_terminal::TerminalReaderDeviceType>,
    id: Option<stripe_terminal::TerminalReaderId>,
    ip_address: Option<Option<String>>,
    label: Option<String>,
    livemode: Option<bool>,
    location: Option<Option<stripe_types::Expandable<stripe_terminal::TerminalLocation>>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    serial_number: Option<String>,
    status: Option<Option<stripe_terminal::TerminalReaderStatus>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                action: Deserialize::default(),
                device_sw_version: Deserialize::default(),
                device_type: Deserialize::default(),
                id: Deserialize::default(),
                ip_address: Deserialize::default(),
                label: Deserialize::default(),
                livemode: Deserialize::default(),
                location: Deserialize::default(),
                metadata: Deserialize::default(),
                serial_number: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                action: self.action.take()?,
                device_sw_version: self.device_sw_version.take()?,
                device_type: self.device_type?,
                id: self.id.take()?,
                ip_address: self.ip_address.take()?,
                label: self.label.take()?,
                livemode: self.livemode?,
                location: self.location.take()?,
                metadata: self.metadata.take()?,
                serial_number: self.serial_number.take()?,
                status: self.status?,
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
                    "action" => b.action = Some(FromValueOpt::from_value(v)?),
                    "device_sw_version" => b.device_sw_version = Some(FromValueOpt::from_value(v)?),
                    "device_type" => b.device_type = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "ip_address" => b.ip_address = Some(FromValueOpt::from_value(v)?),
                    "label" => b.label = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "location" => b.location = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "serial_number" => b.serial_number = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),

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
        let mut s = s.serialize_struct("TerminalReader", 12)?;
        s.serialize_field("action", &self.action)?;
        s.serialize_field("device_sw_version", &self.device_sw_version)?;
        s.serialize_field("device_type", &self.device_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ip_address", &self.ip_address)?;
        s.serialize_field("label", &self.label)?;
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
}
stripe_types::def_id!(TerminalReaderId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    VerifoneP400,
}
impl TerminalReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderDeviceType::*;
        match self {
            BbposChipper2x => "bbpos_chipper2x",
            BbposWisepad3 => "bbpos_wisepad3",
            BbposWiseposE => "bbpos_wisepos_e",
            SimulatedWiseposE => "simulated_wisepos_e",
            StripeM2 => "stripe_m2",
            VerifoneP400 => "verifone_P400",
        }
    }
}

impl std::str::FromStr for TerminalReaderDeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderDeviceType::*;
        match s {
            "bbpos_chipper2x" => Ok(BbposChipper2x),
            "bbpos_wisepad3" => Ok(BbposWisepad3),
            "bbpos_wisepos_e" => Ok(BbposWiseposE),
            "simulated_wisepos_e" => Ok(SimulatedWiseposE),
            "stripe_m2" => Ok(StripeM2),
            "verifone_P400" => Ok(VerifoneP400),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalReaderDeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalReaderDeviceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderDeviceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderDeviceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TerminalReaderDeviceType"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderStatus {
    Offline,
    Online,
}
impl TerminalReaderStatus {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderStatus::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for TerminalReaderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderStatus::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TerminalReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalReaderStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalReaderStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TerminalReaderStatus"))
    }
}
