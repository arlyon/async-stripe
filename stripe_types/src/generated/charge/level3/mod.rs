#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Level3 {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod line_item;
pub use line_item::LineItem;
