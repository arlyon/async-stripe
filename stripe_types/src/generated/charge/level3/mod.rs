#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Level3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    pub line_items: Vec<stripe_types::charge::level3::line_item::LineItem>,
    pub merchant_reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_from_zip: Option<String>,
}
pub mod line_item;
pub use line_item::LineItem;
