#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ShippingDetails {
    /// Recipient shipping address.
    ///
    /// Required if the order includes products that are shippable.
    pub address: Option<stripe_types::address::Address>,
    /// Recipient name.
    pub name: Option<String>,
    /// Recipient phone (including extension).
    pub phone: Option<String>,
}
