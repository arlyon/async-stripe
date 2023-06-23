#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Product {
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    /// The product ID.
    pub product: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Product {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
