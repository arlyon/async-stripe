#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversal {
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: String,
}
