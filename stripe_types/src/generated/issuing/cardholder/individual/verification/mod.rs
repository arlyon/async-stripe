#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Verification {
    /// An identifying document, either a passport or local ID card.
    pub document:
        Option<stripe_types::issuing::cardholder::individual::verification::document::Document>,
}
pub mod document;
pub use document::Document;
