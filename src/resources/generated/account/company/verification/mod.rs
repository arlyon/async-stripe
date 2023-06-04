#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Verification {
    pub document:
        crate::account::company::verification::verification_document::VerificationDocument,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Verification {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod verification_document;
pub use verification_document::VerificationDocument;
