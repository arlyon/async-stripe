#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Breakdown {
    /// The aggregated discounts.
    pub discounts: Vec<crate::line_item::discount::Discount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<crate::line_item::tax::Tax>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Breakdown {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
