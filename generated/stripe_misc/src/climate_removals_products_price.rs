#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClimateRemovalsProductsPrice {
    /// Fees for one metric ton of carbon removal in the currency's smallest unit.
    pub amount_fees: i64,
    /// Subtotal for one metric ton of carbon removal (excluding fees) in the currency's smallest unit.
    pub amount_subtotal: i64,
    /// Total for one metric ton of carbon removal (including fees) in the currency's smallest unit.
    pub amount_total: i64,
}
