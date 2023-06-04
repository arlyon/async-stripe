#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OrderItem {
    /// The amount (price) for this order item.
    pub amount: Option<i64>,
    /// This currency of this order item.
    ///
    /// Required when `amount` is present.
    pub currency: Option<crate::Currency>,
    /// Human-readable description for this order item.
    pub description: Option<String>,
    /// The ID of the associated object for this line item.
    ///
    /// Expandable if not null (e.g., expandable to a SKU).
    pub parent: Option<String>,
    /// The quantity of this order item.
    ///
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The type of this order item.
    ///
    /// Must be `sku`, `tax`, or `shipping`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OrderItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
