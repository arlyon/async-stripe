#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DiscountAmount {
    /// The amount, in %s, of the discount.
    pub amount: i64,
    /// The discount that was applied to get this discount amount.
    pub discount: crate::Expandable<crate::discount::Discount>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DiscountAmount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
