use std::fmt::{Display, Formatter};

/// Error representing a failure to parse a Stripe enum
#[derive(Debug)]
pub struct StripeParseError;

impl Display for StripeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unrecognized enum variant")
    }
}

impl std::error::Error for StripeParseError {}
