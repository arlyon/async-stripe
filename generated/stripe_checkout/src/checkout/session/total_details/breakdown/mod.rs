#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Breakdown {
    /// The aggregated discounts.
    pub discounts: Vec<stripe_types::line_item::discount::Discount>,
    /// The aggregated tax amounts by rate.
    pub taxes: Vec<stripe_types::line_item::tax::Tax>,
}
