#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Reversal {
    /// The `id` of the reversed `Transaction` object.
    pub original_transaction: Option<String>,
}
