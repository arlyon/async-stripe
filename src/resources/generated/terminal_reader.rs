// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::TerminalReaderId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::TerminalLocation;

/// The resource representing a Stripe "TerminalReaderReader".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TerminalReader {
    /// Unique identifier for the object.
    pub id: TerminalReaderId,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The current software version of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_sw_version: Option<String>,

    /// Type of reader, one of `bbpos_chipper2x`, `bbpos_wisepos_e`, or `verifone_P400`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<TerminalReaderDeviceType>,

    /// The local IP address of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// Custom label given to the reader for easier identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// The location identifier of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Expandable<TerminalLocation>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Serial number of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// The networking status of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Object for TerminalReader {
    type Id = TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "terminal.reader"
    }
}

/// An enum representing the possible values of an `TerminalReader`'s `device_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderDeviceType {
    BbposChipper2x,
    BbposWiseposE,
    #[serde(rename = "verifone_P400")]
    VerifoneP400,
}

impl TerminalReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalReaderDeviceType::BbposChipper2x => "bbpos_chipper2x",
            TerminalReaderDeviceType::BbposWiseposE => "bbpos_wisepos_e",
            TerminalReaderDeviceType::VerifoneP400 => "verifone_P400",
        }
    }
}

impl AsRef<str> for TerminalReaderDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
