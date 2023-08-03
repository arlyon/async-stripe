/// Configures how this subscription behaves during the trial period.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsTrialsResourceTrialSettings {
    pub end_behavior: stripe_types::SubscriptionsTrialsResourceEndBehavior,
}
