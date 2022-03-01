// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationMerchantData".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MerchantData {
    /// A categorization of the seller's type of business.
    ///
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,

    /// The merchant category code for the seller’s business.
    pub category_code: String,

    /// City where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<String>>,

    /// Country where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Name of the seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// Identifier assigned to the seller by the card brand.
    pub network_id: String,

    /// Postal code where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Box<String>>,

    /// State where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<String>>,
}
