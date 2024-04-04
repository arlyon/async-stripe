#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodCardPresentNetworks {
    /// All available networks for the card.
    pub available: Vec<String>,
    /// The preferred network for the card.
    pub preferred: Option<String>,
}
