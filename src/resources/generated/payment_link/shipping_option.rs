#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    /// The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: crate::Expandable<crate::shipping_rate::ShippingRate>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ShippingOption {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
