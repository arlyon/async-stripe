/// Represents a line item to be displayed on the reader.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LineItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
