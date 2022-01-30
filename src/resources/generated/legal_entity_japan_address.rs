// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "LegalEntityJapanAddress".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    /// City/Ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<String>>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Block/Building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<Box<String>>,

    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<Box<String>>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Box<String>>,

    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<String>>,

    /// Town/cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<Box<String>>,
}
