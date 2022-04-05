//! Stripe removed RecipientType from the openapi
//! definitons in favor of just a string value. It
//! is still, however limited to these two cases,
//! so instead we just define the enum ourselves.

use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `ListRecipients`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecipientType {
    Corporation,
    Individual,
}

impl RecipientType {
    pub fn as_str(self) -> &'static str {
        match self {
            RecipientType::Corporation => "corporation",
            RecipientType::Individual => "individual",
        }
    }
}

impl AsRef<str> for RecipientType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecipientType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RecipientType {
    fn default() -> Self {
        Self::Corporation
    }
}
