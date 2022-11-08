// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "LegalEntityJapanAddress".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {

    /// City/Ward.
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,

    /// Block/Building number.
    pub line1: Option<String>,

    /// Building details.
    pub line2: Option<String>,

    /// ZIP or postal code.
    pub postal_code: Option<String>,

    /// Prefecture.
    pub state: Option<String>,

    /// Town/cho-me.
    pub town: Option<String>,
}
