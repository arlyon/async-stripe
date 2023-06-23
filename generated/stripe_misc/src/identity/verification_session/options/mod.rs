#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_misc::identity::verification_session::options::document::Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number:
        Option<stripe_misc::identity::verification_session::options::id_number::IdNumber>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Options {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod document;
pub use document::Document;
pub mod id_number;
pub use id_number::IdNumber;
