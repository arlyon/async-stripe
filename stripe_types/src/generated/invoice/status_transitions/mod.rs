#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatusTransitions {
    /// The time that the invoice draft was finalized.
    pub finalized_at: Option<stripe_types::Timestamp>,
    /// The time that the invoice was marked uncollectible.
    pub marked_uncollectible_at: Option<stripe_types::Timestamp>,
    /// The time that the invoice was paid.
    pub paid_at: Option<stripe_types::Timestamp>,
    /// The time that the invoice was voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
