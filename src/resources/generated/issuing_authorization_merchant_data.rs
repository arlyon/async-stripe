// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationMerchantData".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MerchantData {
    /// A categorization of the seller's type of business.
    ///
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,

    /// The merchant category code for the seller’s business.
    pub category_code: String,

    /// City where the seller is located.
    pub city: Box<Option<String>>,

    /// Country where the seller is located.
    pub country: Box<Option<String>>,

    /// Name of the seller.
    pub name: Box<Option<String>>,

    /// Identifier assigned to the seller by the card brand.
    pub network_id: String,

    /// Postal code where the seller is located.
    pub postal_code: Box<Option<String>>,

    /// State where the seller is located.
    pub state: Box<Option<String>>,
}
