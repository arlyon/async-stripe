#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceRestrictions {
    pub completed_sessions: stripe_shared::PaymentLinksResourceCompletedSessions,
}
