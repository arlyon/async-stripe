#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<stripe_types::status_details_blocked::StatusDetailsBlocked>,
}
