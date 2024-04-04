#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotesResourceFromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,
    /// The quote that was cloned.
    pub quote: stripe_types::Expandable<stripe_shared::Quote>,
}
