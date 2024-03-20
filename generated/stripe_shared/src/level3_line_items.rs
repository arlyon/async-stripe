#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Level3LineItems {
    pub discount_amount: Option<i64>,
    pub product_code: String,
    pub product_description: String,
    pub quantity: Option<u64>,
    pub tax_amount: Option<i64>,
    pub unit_cost: Option<i64>,
}
