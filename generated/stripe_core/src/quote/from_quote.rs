#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,
    /// The quote that was cloned.
    pub quote: stripe_types::Expandable<stripe_core::quote::Quote>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FromQuote {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
