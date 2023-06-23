#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Discount {
    /// The amount discounted.
    pub amount: i64,
    pub discount: stripe_core::discount::Discount,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Discount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
