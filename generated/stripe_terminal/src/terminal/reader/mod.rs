/// A Reader represents a physical device for accepting payment details.
///
/// Related guide: [Connecting to a Reader](https://stripe.com/docs/terminal/payments/connect-reader).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Reader {
    /// The most recent action performed by the reader.
    pub action: Option<stripe_terminal::terminal::reader::reader_action::ReaderAction>,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Reader {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    #[serde(rename = "verifone_P400")]
    VerifoneP400,
}

impl ReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BbposChipper2x => "bbpos_chipper2x",
            Self::BbposWisepad3 => "bbpos_wisepad3",
            Self::BbposWiseposE => "bbpos_wisepos_e",
            Self::SimulatedWiseposE => "simulated_wisepos_e",
            Self::StripeM2 => "stripe_m2",
            Self::VerifoneP400 => "verifone_P400",
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReaderObject {
    #[serde(rename = "terminal.reader")]
    TerminalReader,
}

impl ReaderObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TerminalReader => "terminal.reader",
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
impl stripe_types::Object for Reader {
    type Id = stripe_terminal::terminal::reader::TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TerminalReaderId, "tmr_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedReader;
pub mod reader_action;
pub use reader_action::ReaderAction;
