// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::Address;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Shipping".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Shipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}
