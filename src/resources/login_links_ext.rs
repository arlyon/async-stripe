use serde_derive::Serialize;

use crate::config::{Client, Response};
use crate::params::Expand;
use crate::resources::LoginLink;
use crate::AccountId;

pub trait CreateLoginLinkExt {
    fn create(client: &Client, id: &AccountId, redirect_url: &str) -> Response<Self>
    where
        Self: Sized;

    #[cfg(features = "idempotency")]
    fn create_with_idempotency(
        client: &Client,
        id: &AccountId,
        redirect_url: &str,
        idem_key: &str,
    ) -> Response<Self>
    where
        Self: Sized;
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateLoginLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Where to redirect the user after they log out of their dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl CreateLoginLinkExt for LoginLink {
    fn create(client: &Client, id: &AccountId, redirect_url: &str) -> Response<Self> {
        let create_login_link =
            CreateLoginLink { expand: &[], redirect_url: Some(redirect_url.to_string()) };

        client.post_form(&format!("/accounts/{}/login_links", id), &create_login_link, None)
    }

    #[cfg(features = "idempotency")]
    fn create_with_idempotency(
        client: &Client,
        id: &AccountId,
        redirect_url: &str,
        idem_key: &str,
    ) -> Response<Self> {
        let create_login_link =
            CreateLoginLink { expand: &[], redirect_url: Some(redirect_url.to_string()) };

        client.post_form(
            &format!("/accounts/{}/login_links", id),
            &create_login_link,
            Some(idem_key),
        )
    }
}
