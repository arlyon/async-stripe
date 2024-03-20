#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotesResourceUpfront {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The line items that will appear on the next invoice after this quote is accepted.
    /// This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    pub total_details: stripe_shared::QuotesResourceTotalDetails,
}
