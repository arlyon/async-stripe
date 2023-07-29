#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<stripe_types::Expandable<stripe_types::charge::Charge>>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_types::setup_attempt::SetupAttempt>>,
}
