// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PackageDimensions".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PackageDimensions {
    /// Height, in inches.
    pub height: f64,

    /// Length, in inches.
    pub length: f64,

    /// Weight, in ounces.
    pub weight: f64,

    /// Width, in inches.
    pub width: f64,
}
