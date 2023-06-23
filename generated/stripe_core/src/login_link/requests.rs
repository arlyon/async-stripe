use stripe::{Client, Response};

impl stripe_core::login_link::LoginLink {
    /// Creates a single-use login link for an Express account to access their Stripe dashboard.
    ///
    /// **You may only create login links for [Express accounts](https://stripe.com/docs/connect/express-accounts) connected to your platform**.
    pub fn create(
        client: &Client,
        account: &stripe_types::AccountId,
        params: CreateLoginLink,
    ) -> Response<stripe_core::login_link::LoginLink> {
        client.send_form(
            &format!("/accounts/{account}/login_links", account = account),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateLoginLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateLoginLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}