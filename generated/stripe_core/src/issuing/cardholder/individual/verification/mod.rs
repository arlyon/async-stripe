#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Verification {
    /// An identifying document, either a passport or local ID card.
    pub document:
        Option<stripe_core::issuing::cardholder::individual::verification::document::Document>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Verification {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod document;
pub use document::Document;
