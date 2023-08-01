#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Reversal {
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: String,
}
