#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountTreasurySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_shared::AccountTermsOfService>,
}
