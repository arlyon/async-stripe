#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_misc::identity::verification_report::options::document::Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<stripe_misc::identity::verification_report::options::id_number::IdNumber>,
}
pub mod document;
pub use document::Document;
pub mod id_number;
pub use id_number::IdNumber;
