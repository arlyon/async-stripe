use std::fmt::Display;

use stripe_client_core::StripeClientErr;
use stripe_shared::ApiErrors;
use thiserror::Error;

/// An error encountered when communicating with the Stripe API.
#[derive(Debug, Error)]
pub enum StripeError {
    /// Stripe returned a client error.
    #[error("error reported by stripe: {0:#?}, status code: {1}")]
    Stripe(Box<ApiErrors>, u16),
    /// An error occurred when parsing the Stripe response.
    #[error("error deserializing a request: {0}")]
    JSONDeserialize(String),
    /// An error occurred communicating with Stripe.
    #[error("error communicating with stripe: {0}")]
    ClientError(String),
    /// The client configuration was invalid.
    #[error("configuration error: {0}")]
    ConfigError(String),
    /// A blocking request timed out
    #[error("timeout communicating with stripe")]
    Timeout,
}

impl StripeClientErr for StripeError {
    fn deserialize_err(msg: impl Display) -> Self {
        Self::JSONDeserialize(msg.to_string())
    }
}

impl From<hyper::Error> for StripeError {
    fn from(err: hyper::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

impl From<hyper::http::Error> for StripeError {
    fn from(err: hyper::http::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}
