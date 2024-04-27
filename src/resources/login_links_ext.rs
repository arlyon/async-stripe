use serde::Serialize;

use crate::client::{Client, Response};
use crate::params::Expand;
use crate::resources::LoginLink;
use crate::AccountId;

#[derive(Clone, Debug, Serialize)]
pub struct CreateLoginLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Where to redirect the user after they log out of their dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl LoginLink {
    pub fn create(client: &Client, id: &AccountId, redirect_url: &str) -> Response<Self> {
        let create_login_link =
            CreateLoginLink { expand: &[], redirect_url: Some(redirect_url.to_string()) };

        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/accounts/{}/login_links", id), &create_login_link)
    }
}
