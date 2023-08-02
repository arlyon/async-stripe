#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SettingsCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_types::tos_acceptance::TosAcceptance>,
}
