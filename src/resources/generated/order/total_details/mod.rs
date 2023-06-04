#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<crate::order::total_details::breakdown::Breakdown>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TotalDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod breakdown;
pub use breakdown::Breakdown;
