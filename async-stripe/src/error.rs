use stripe_types::ApiErrors;
use thiserror::Error;

/// An error encountered when communicating with the Stripe API.
#[derive(Debug, Error)]
pub enum StripeError {
    #[error("error reported by stripe: {0:#?}, status code: {1}")]
    Stripe(ApiErrors, u16),
    #[error("error serializing or deserializing a querystring: {0}")]
    QueryStringSerialize(#[from] serde_path_to_error::Error<serde_qs::Error>),
    #[error("error serializing or deserializing a request")]
    JSONSerialize(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error("attempted to access an unsupported version of the api")]
    UnsupportedVersion,
    #[error("error communicating with stripe: {0}")]
    ClientError(String),
    #[error("timeout communicating with stripe")]
    Timeout,
}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for StripeError {
    fn from(err: hyper::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

impl From<http_types::Error> for StripeError {
    fn from(err: http_types::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}
