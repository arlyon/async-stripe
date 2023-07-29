#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SettingsTreasury {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance:
        Option<stripe_types::account::settings::settings_treasury::tos_acceptance::TosAcceptance>,
}
pub mod tos_acceptance;
pub use tos_acceptance::TosAcceptance;
