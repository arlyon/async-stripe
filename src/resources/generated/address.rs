// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Address".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    pub line2: Option<String>,

    /// ZIP or postal code.
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    pub state: Option<String>,
}
