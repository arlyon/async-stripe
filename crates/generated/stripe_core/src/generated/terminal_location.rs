// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_core::{
    ids::TerminalLocationId,
    params::{Metadata, Object},
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TerminalLocationLocation".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalLocation {
    /// Unique identifier for the object.
    pub id: TerminalLocationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The display name of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
