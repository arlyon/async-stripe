// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::TerminalLocationId;
use crate::params::{Metadata, Object};
use crate::resources::Address;

/// The resource representing a Stripe "TerminalLocationLocation".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TerminalLocation {
    /// Unique identifier for the object.
    pub id: TerminalLocationId,

    pub address: Box<Option<Address>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The display name of the location.
    pub display_name: Box<Option<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: Box<Option<bool>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,
}

impl Object for TerminalLocation {
    type Id = TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "terminal.location"
    }
}
