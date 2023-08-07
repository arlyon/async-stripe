use std::num::ParseIntError;

/// An error encountered when communicating with the Stripe API webhooks.
#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("invalid key length")]
    BadKey,
    #[error("error parsing timestamp")]
    BadHeader(#[from] ParseIntError),
    #[error("error comparing signatures")]
    BadSignature,
    #[error("error comparing timestamps - over tolerance")]
    BadTimestamp(i64),
    #[error("error parsing event object")]
    BadParse(#[from] serde_json::Error),
}
