#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MandateAuBecsDebit {
    /// The URL of the mandate.
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
