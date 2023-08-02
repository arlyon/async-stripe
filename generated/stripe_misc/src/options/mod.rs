#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_misc::document::Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_misc::id_number::IdNumber>,
}
