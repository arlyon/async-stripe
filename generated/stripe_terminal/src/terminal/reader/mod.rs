/// A Reader represents a physical device for accepting payment details.
///
/// Related guide: [Connecting to a reader](https://stripe.com/docs/terminal/payments/connect-reader).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Reader {
    /// The most recent action performed by the reader.
    pub action: Option<stripe_terminal::reader_action::ReaderAction>,
    /// The current software version of the reader.
    pub device_sw_version: Option<String>,
    /// Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`.
    pub device_type: ReaderDeviceType,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal::reader::TerminalReaderId,
    /// The local IP address of the reader.
    pub ip_address: Option<String>,
    /// Custom label given to the reader for easier identification.
    pub label: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The location identifier of the reader.
    pub location: Option<stripe_types::Expandable<stripe_terminal::terminal::location::Location>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReaderObject,
    /// Serial number of the reader.
    pub serial_number: String,
    /// The networking status of the reader.
    pub status: Option<String>,
}
/// Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    VerifoneP400,
}

impl ReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        use ReaderDeviceType::*;
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

impl std::str::FromStr for ReaderDeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReaderDeviceType::*;
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

impl AsRef<str> for ReaderDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReaderDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReaderDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReaderDeviceType"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReaderObject {
    TerminalReader,
}

impl ReaderObject {
    pub fn as_str(self) -> &'static str {
        use ReaderObject::*;
        match self {
            TerminalReader => "terminal.reader",
        }
    }
}

impl std::str::FromStr for ReaderObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReaderObject::*;
        match s {
            "terminal.reader" => Ok(TerminalReader),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReaderObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReaderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReaderObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReaderObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for ReaderObject"))
    }
}
impl stripe_types::Object for Reader {
    type Id = stripe_terminal::terminal::reader::TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TerminalReaderId, "tmr_");
pub mod deleted;
pub use deleted::DeletedReader;
pub mod requests;
