use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListAccountCapabilityBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAccountCapabilityBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAccountCapabilityBuilder").finish_non_exhaustive()
    }
}
impl ListAccountCapabilityBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a list of capabilities associated with the account.
/// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListAccountCapability {
    inner: ListAccountCapabilityBuilder,
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAccountCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAccountCapability").finish_non_exhaustive()
    }
}
impl ListAccountCapability {
    /// Construct a new `ListAccountCapability`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: ListAccountCapabilityBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ListAccountCapability {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Capability>> {
        let account = &self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/capabilities"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListAccountCapability {
    type Output = stripe_types::List<stripe_shared::Capability>;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/capabilities"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveCapabilityBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCapabilityBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCapabilityBuilder").finish_non_exhaustive()
    }
}
impl RetrieveCapabilityBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves information about the specified Account Capability.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveCapability {
    inner: RetrieveCapabilityBuilder,
    account: stripe_shared::AccountId,
    capability: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveCapability").finish_non_exhaustive()
    }
}
impl RetrieveCapability {
    /// Construct a new `RetrieveCapability`.
    pub fn new(
        account: impl Into<stripe_shared::AccountId>,
        capability: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            capability: capability.into(),
            inner: RetrieveCapabilityBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveCapability {
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

impl StripeRequest for RetrieveCapability {
    type Output = stripe_shared::Capability;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let capability = &self.capability;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/accounts/{account}/capabilities/{capability}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateCapabilityBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCapabilityBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCapabilityBuilder").finish_non_exhaustive()
    }
}
impl UpdateCapabilityBuilder {
    fn new() -> Self {
        Self { expand: None, requested: None }
    }
}
/// Updates an existing Account Capability.
/// Request or remove a capability by updating its `requested` parameter.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateCapability {
    inner: UpdateCapabilityBuilder,
    account: stripe_shared::AccountId,
    capability: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateCapability").finish_non_exhaustive()
    }
}
impl UpdateCapability {
    /// Construct a new `UpdateCapability`.
    pub fn new(
        account: impl Into<stripe_shared::AccountId>,
        capability: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            capability: capability.into(),
            inner: UpdateCapabilityBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// To request a new capability for an account, pass true.
    /// There can be a delay before the requested capability becomes active.
    /// If the capability has any activation requirements, the response includes them in the `requirements` arrays.
    ///
    /// If a capability isn't permanent, you can remove it from the account by passing false.
    /// Some capabilities are permanent after they've been requested.
    /// Attempting to remove a permanent capability returns an error.
    pub fn requested(mut self, requested: impl Into<bool>) -> Self {
        self.inner.requested = Some(requested.into());
        self
    }
}
impl UpdateCapability {
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

impl StripeRequest for UpdateCapability {
    type Output = stripe_shared::Capability;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        let capability = &self.capability;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/accounts/{account}/capabilities/{capability}"),
        )
        .form(&self.inner)
    }
}
