// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "LegalEntityJapanAddress".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    /// City/Ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Block/Building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Town/cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
