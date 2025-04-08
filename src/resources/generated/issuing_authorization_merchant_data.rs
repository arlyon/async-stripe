// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

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
    pub city: Option<String>,

    /// Country where the seller is located.
    pub country: Option<String>,

    /// Name of the seller.
    pub name: Option<String>,

    /// Identifier assigned to the seller by the card network.
    ///
    /// Different card networks may assign different network_id fields to the same merchant.
    pub network_id: String,

    /// Postal code where the seller is located.
    pub postal_code: Option<String>,

    /// State where the seller is located.
    pub state: Option<String>,

    /// The seller's tax identification number.
    ///
    /// Currently populated for French merchants only.
    pub tax_id: Option<String>,

    /// An ID assigned by the seller to the location of the sale.
    pub terminal_id: Option<String>,

    /// URL provided by the merchant on a 3DS request.
    pub url: Option<String>,
}
