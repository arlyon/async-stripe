// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "LegalEntityJapanAddress".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    /// City/Ward.
    pub city: Box<Option<String>>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Box<Option<String>>,

    /// Block/Building number.
    pub line1: Box<Option<String>>,

    /// Building details.
    pub line2: Box<Option<String>>,

    /// ZIP or postal code.
    pub postal_code: Box<Option<String>>,

    /// Prefecture.
    pub state: Box<Option<String>>,

    /// Town/cho-me.
    pub town: Box<Option<String>>,
}
