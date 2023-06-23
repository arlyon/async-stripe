#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
