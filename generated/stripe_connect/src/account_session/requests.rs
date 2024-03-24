#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSession<'a> {
    /// The identifier of the account to create an Account Session for.
    pub account: &'a str,
    /// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
    /// whether it has been enabled or not).
    pub components: CreateAccountSessionComponents,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountSession<'a> {
    pub fn new(account: &'a str, components: CreateAccountSessionComponents) -> Self {
        Self { account, components, expand: None }
    }
}
/// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
/// whether it has been enabled or not).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSessionComponents {
    /// Configuration for the account onboarding embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<CreateAccountSessionComponentsAccountOnboarding>,
}
impl CreateAccountSessionComponents {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the account onboarding embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboarding {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
}
impl CreateAccountSessionComponentsAccountOnboarding {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
impl<'a> CreateAccountSession<'a> {
    /// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_connect::AccountSession> {
        client.send_form("/account_sessions", self, http_types::Method::Post)
    }
}
