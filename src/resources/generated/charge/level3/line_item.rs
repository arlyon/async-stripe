#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LineItem {
    pub discount_amount: Option<i64>,
    pub product_code: String,
    pub product_description: String,
    pub quantity: Option<u64>,
    pub tax_amount: Option<i64>,
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LineItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
