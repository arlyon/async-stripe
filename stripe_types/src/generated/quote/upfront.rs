#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Upfront {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The line items that will appear on the next invoice after this quote is accepted.
    ///
    /// This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    #[serde(default)]
    pub line_items: stripe_types::List<stripe_types::line_item::LineItem>,
    pub total_details: stripe_types::quote::total_details::TotalDetails,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Upfront {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
