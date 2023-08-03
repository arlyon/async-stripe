#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Level3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    pub line_items: Vec<stripe_types::Level3LineItems>,
    pub merchant_reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_from_zip: Option<String>,
}
