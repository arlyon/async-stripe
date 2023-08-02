#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<stripe_misc::active::Active>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<stripe_misc::pending::Pending>,
}
