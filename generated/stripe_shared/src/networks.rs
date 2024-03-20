#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Networks {
    /// All available networks for the card.
    pub available: Vec<String>,
    /// The preferred network for co-branded cards.
    /// Can be `cartes_bancaires`, `mastercard`, `visa` or `invalid_preference` if requested network is not valid for the card.
    pub preferred: Option<String>,
}
