#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types::address::Address>,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}