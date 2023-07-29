/// Represents a line item to be displayed on the reader.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LineItem {
    /// The amount of the line item.
    ///
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Description of the line item.
    pub description: String,
    /// The quantity of the line item.
    pub quantity: u64,
}
