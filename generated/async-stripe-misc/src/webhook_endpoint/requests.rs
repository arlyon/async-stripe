use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// You can also delete webhook endpoints via the [webhook endpoint management](https://dashboard.stripe.com/account/webhooks) page of the Stripe dashboard.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteWebhookEndpoint {
    webhook_endpoint: stripe_misc::WebhookEndpointId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteWebhookEndpoint").finish_non_exhaustive()
    }
}
impl DeleteWebhookEndpoint {
    /// Construct a new `DeleteWebhookEndpoint`.
    pub fn new(webhook_endpoint: impl Into<stripe_misc::WebhookEndpointId>) -> Self {
        Self { webhook_endpoint: webhook_endpoint.into() }
    }
}
impl DeleteWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteWebhookEndpoint {
    type Output = stripe_misc::DeletedWebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        let webhook_endpoint = &self.webhook_endpoint;
        RequestBuilder::new(StripeMethod::Delete, format!("/webhook_endpoints/{webhook_endpoint}"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListWebhookEndpointBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListWebhookEndpointBuilder").finish_non_exhaustive()
    }
}
impl ListWebhookEndpointBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your webhook endpoints.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListWebhookEndpoint {
    inner: ListWebhookEndpointBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListWebhookEndpoint").finish_non_exhaustive()
    }
}
impl ListWebhookEndpoint {
    /// Construct a new `ListWebhookEndpoint`.
    pub fn new() -> Self {
        Self { inner: ListWebhookEndpointBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListWebhookEndpoint {
    fn default() -> Self {
        Self::new()
    }
}
impl ListWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::WebhookEndpoint>> {
        stripe_client_core::ListPaginator::new_list("/webhook_endpoints", &self.inner)
    }
}

impl StripeRequest for ListWebhookEndpoint {
    type Output = stripe_types::List<stripe_misc::WebhookEndpoint>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/webhook_endpoints").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveWebhookEndpointBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveWebhookEndpointBuilder").finish_non_exhaustive()
    }
}
impl RetrieveWebhookEndpointBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the webhook endpoint with the given ID.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveWebhookEndpoint {
    inner: RetrieveWebhookEndpointBuilder,
    webhook_endpoint: stripe_misc::WebhookEndpointId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveWebhookEndpoint").finish_non_exhaustive()
    }
}
impl RetrieveWebhookEndpoint {
    /// Construct a new `RetrieveWebhookEndpoint`.
    pub fn new(webhook_endpoint: impl Into<stripe_misc::WebhookEndpointId>) -> Self {
        Self {
            webhook_endpoint: webhook_endpoint.into(),
            inner: RetrieveWebhookEndpointBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveWebhookEndpoint {
    type Output = stripe_misc::WebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        let webhook_endpoint = &self.webhook_endpoint;
        RequestBuilder::new(StripeMethod::Get, format!("/webhook_endpoints/{webhook_endpoint}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<stripe_shared::ApiVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    enabled_events: Vec<stripe_shared::EnabledEvents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateWebhookEndpointBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateWebhookEndpointBuilder").finish_non_exhaustive()
    }
}
impl CreateWebhookEndpointBuilder {
    fn new(
        enabled_events: impl Into<Vec<stripe_shared::EnabledEvents>>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            api_version: None,
            connect: None,
            description: None,
            enabled_events: enabled_events.into(),
            expand: None,
            metadata: None,
            url: url.into(),
        }
    }
}
/// A webhook endpoint must have a `url` and a list of `enabled_events`.
/// You may optionally specify the Boolean `connect` parameter.
/// If set to true, then a Connect webhook endpoint that notifies the specified `url` about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified `url` only about events from your account is created.
/// You can also create webhook endpoints in the [webhooks settings](https://dashboard.stripe.com/account/webhooks) section of the Dashboard.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateWebhookEndpoint {
    inner: CreateWebhookEndpointBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateWebhookEndpoint").finish_non_exhaustive()
    }
}
impl CreateWebhookEndpoint {
    /// Construct a new `CreateWebhookEndpoint`.
    pub fn new(
        enabled_events: impl Into<Vec<stripe_shared::EnabledEvents>>,
        url: impl Into<String>,
    ) -> Self {
        Self { inner: CreateWebhookEndpointBuilder::new(enabled_events.into(), url.into()) }
    }
    /// Events sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version.
    pub fn api_version(mut self, api_version: impl Into<stripe_shared::ApiVersion>) -> Self {
        self.inner.api_version = Some(api_version.into());
        self
    }
    /// Whether this endpoint should receive events from connected accounts (`true`), or from your account (`false`).
    /// Defaults to `false`.
    pub fn connect(mut self, connect: impl Into<bool>) -> Self {
        self.inner.connect = Some(connect.into());
        self
    }
    /// An optional description of what the webhook is used for.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateWebhookEndpoint {
    type Output = stripe_misc::WebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/webhook_endpoints").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_events: Option<Vec<stripe_shared::EnabledEvents>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateWebhookEndpointBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateWebhookEndpointBuilder").finish_non_exhaustive()
    }
}
impl UpdateWebhookEndpointBuilder {
    fn new() -> Self {
        Self {
            description: None,
            disabled: None,
            enabled_events: None,
            expand: None,
            metadata: None,
            url: None,
        }
    }
}
/// Updates the webhook endpoint.
/// You may edit the `url`, the list of `enabled_events`, and the status of your endpoint.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateWebhookEndpoint {
    inner: UpdateWebhookEndpointBuilder,
    webhook_endpoint: stripe_misc::WebhookEndpointId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateWebhookEndpoint").finish_non_exhaustive()
    }
}
impl UpdateWebhookEndpoint {
    /// Construct a new `UpdateWebhookEndpoint`.
    pub fn new(webhook_endpoint: impl Into<stripe_misc::WebhookEndpointId>) -> Self {
        Self {
            webhook_endpoint: webhook_endpoint.into(),
            inner: UpdateWebhookEndpointBuilder::new(),
        }
    }
    /// An optional description of what the webhook is used for.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Disable the webhook endpoint if set to true.
    pub fn disabled(mut self, disabled: impl Into<bool>) -> Self {
        self.inner.disabled = Some(disabled.into());
        self
    }
    /// The list of events to enable for this endpoint.
    /// You may specify `['*']` to enable all events, except those that require explicit selection.
    pub fn enabled_events(
        mut self,
        enabled_events: impl Into<Vec<stripe_shared::EnabledEvents>>,
    ) -> Self {
        self.inner.enabled_events = Some(enabled_events.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The URL of the webhook endpoint.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner.url = Some(url.into());
        self
    }
}
impl UpdateWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateWebhookEndpoint {
    type Output = stripe_misc::WebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        let webhook_endpoint = &self.webhook_endpoint;
        RequestBuilder::new(StripeMethod::Post, format!("/webhook_endpoints/{webhook_endpoint}"))
            .form(&self.inner)
    }
}

/// The list of events to enable for this endpoint.
/// You may specify `['*']` to enable all events, except those that require explicit selection.
pub type CreateWebhookEndpointEnabledEvents = stripe_shared::EnabledEvents;
/// The list of events to enable for this endpoint.
/// You may specify `['*']` to enable all events, except those that require explicit selection.
pub type UpdateWebhookEndpointEnabledEvents = stripe_shared::EnabledEvents;
