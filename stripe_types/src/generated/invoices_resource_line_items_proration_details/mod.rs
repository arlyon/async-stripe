#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoicesResourceLineItemsProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<stripe_types::InvoicesResourceLineItemsCreditedItems>,
}
