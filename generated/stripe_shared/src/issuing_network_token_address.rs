#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingNetworkTokenAddress {
    /// The street address of the cardholder tokenizing the card.
    pub line1: String,
    /// The postal code of the cardholder tokenizing the card.
    pub postal_code: String,
}
