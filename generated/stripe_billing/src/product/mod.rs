#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Product {
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    /// The product ID.
    pub product: String,
}
