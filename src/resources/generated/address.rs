// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Address".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    pub city: Box<Option<String>>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Box<Option<String>>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: Box<Option<String>>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    pub line2: Box<Option<String>>,

    /// ZIP or postal code.
    pub postal_code: Box<Option<String>>,

    /// State, county, province, or region.
    pub state: Box<Option<String>>,
}
