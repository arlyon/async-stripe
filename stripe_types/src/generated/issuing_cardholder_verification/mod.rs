#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderVerification {
    /// An identifying document, either a passport or local ID card.
    pub document: Option<stripe_types::IssuingCardholderIdDocument>,
}
