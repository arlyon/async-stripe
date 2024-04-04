#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoVerificationSessionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_misc::GelatoSessionDocumentOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_misc::GelatoSessionIdNumberOptions>,
}
