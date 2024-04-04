#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxTransactionResourceReversal {
    /// The `id` of the reversed `Transaction` object.
    pub original_transaction: Option<String>,
}
