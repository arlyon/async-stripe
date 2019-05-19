use serde_derive::{Deserialize, Serialize};

/// The set of parameters that can be used when creating an account for users.
///
/// For more details see https://stripe.com/docs/api#create_account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>, // (country the account holder resides in)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>, // (required if account type is standard)
    #[serde(rename = "type")]
    pub account_type: &'static str,
}
