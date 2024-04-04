/// A Reader represents a physical device for accepting payment details.
///
/// Related guide: [Connecting to a reader](https://stripe.com/docs/terminal/payments/connect-reader)
///
/// For more details see <<https://stripe.com/docs/api/terminal/readers/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
impl stripe_types::Object for TerminalReader {
    type Id = stripe_terminal::TerminalReaderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TerminalReaderId, "tmr_");
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
impl<'de> serde::Deserialize<'de> for TerminalReaderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TerminalReaderStatus"))
    }
}
