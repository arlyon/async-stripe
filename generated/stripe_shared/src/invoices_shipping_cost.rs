#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoicesShippingCost {
    /// Total shipping cost before any taxes are applied.
    pub amount_subtotal: i64,
    /// Total tax amount applied due to shipping costs. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    /// Total shipping cost after taxes are applied.
    pub amount_total: i64,
    /// The ID of the ShippingRate for this invoice.
    pub shipping_rate: Option<stripe_types::Expandable<stripe_shared::ShippingRate>>,
    /// The taxes applied to the shipping rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}
