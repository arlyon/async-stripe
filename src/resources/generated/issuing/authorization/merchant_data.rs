#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
    /// Identifier assigned to the seller by the card brand.
    pub network_id: String,
    /// Postal code where the seller is located.
    pub postal_code: Option<String>,
    /// State where the seller is located.
    pub state: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MerchantData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
