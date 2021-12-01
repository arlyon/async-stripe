// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::resources::Address;

/// The resource representing a Stripe "Shipping".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shipping {
    pub address: Box<Option<Address>>,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    pub carrier: Box<Option<String>>,

    /// Recipient name.
    pub name: Box<Option<String>>,

    /// Recipient phone (including extension).
    pub phone: Box<Option<String>>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub tracking_number: Box<Option<String>>,
}
