#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSession<'a> {
    /// The identifier of the account to create an Account Session for.
    pub account: &'a str,
    /// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
    ///
    /// whether it has been enabled or not).
    pub components: CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponents,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSession<'a> {
    pub fn new(
        account: &'a str,
        components: CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponents,
    ) -> Self {
        Self { account, components, expand: Default::default() }
    }
}
/// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
///
/// whether it has been enabled or not).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponents {
    /// Configuration for the account onboarding embedded component.
#[serde(skip_serializing_if = "Option::is_none")]
pub account_onboarding: Option<CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponentsAccountOnboarding>,

}
impl CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponents {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the account onboarding embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponentsAccountOnboarding
{
    /// Whether the embedded component is enabled.
    pub enabled: bool,
}
impl
    CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSessionComponentsAccountOnboarding
{
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
impl<'a> CreateConnectEmbeddedMethodAccountSessionCreateMethodAccountSession<'a> {
    /// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<
        stripe_connect::ConnectEmbeddedMethodAccountSessionCreateMethodAccountSession,
    > {
        client.send_form("/account_sessions", self, http_types::Method::Post)
    }
}
