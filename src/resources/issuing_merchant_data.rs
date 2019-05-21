use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationMerchantData".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MerchantData {
    /// Identifier assigned to the seller by the card brand.
    pub network_id: String,

    /// A categorization of the seller's type of business.
    ///
    /// See the [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,

    /// Name of the seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// City where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// State where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Country where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Postal code where the seller is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}
