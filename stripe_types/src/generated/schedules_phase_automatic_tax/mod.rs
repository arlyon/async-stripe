#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SchedulesPhaseAutomaticTax {
    /// Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
}
