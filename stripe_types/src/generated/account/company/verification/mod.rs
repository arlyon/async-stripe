#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Verification {
    pub document:
        stripe_types::account::company::verification::verification_document::VerificationDocument,
}
pub mod verification_document;
pub use verification_document::VerificationDocument;
