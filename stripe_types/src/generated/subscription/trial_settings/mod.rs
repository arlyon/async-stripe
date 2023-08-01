/// Configures how this subscription behaves during the trial period.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrialSettings {
    pub end_behavior: stripe_types::subscription::trial_settings::end_behavior::EndBehavior,
}
pub mod end_behavior;
pub use end_behavior::EndBehavior;
