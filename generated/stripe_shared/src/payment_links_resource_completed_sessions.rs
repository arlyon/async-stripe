#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceCompletedSessions {
    /// The current number of checkout sessions that have been completed on the payment link which count towards the `completed_sessions` restriction to be met.
    pub count: u64,
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}
