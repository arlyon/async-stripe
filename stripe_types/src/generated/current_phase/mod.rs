#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: stripe_types::Timestamp,
    /// The start of this phase of the subscription schedule.
    pub start_date: stripe_types::Timestamp,
}
