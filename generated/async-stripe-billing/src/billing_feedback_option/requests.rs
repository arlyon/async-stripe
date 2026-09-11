use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListBillingFeedbackOptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::BillingFeedbackOptionStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListBillingFeedbackOptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListBillingFeedbackOptionBuilder").finish_non_exhaustive()
    }
}
impl ListBillingFeedbackOptionBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, status: None }
    }
}
/// An API method for listing the feedback options model
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListBillingFeedbackOption {
    inner: ListBillingFeedbackOptionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListBillingFeedbackOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListBillingFeedbackOption").finish_non_exhaustive()
    }
}
impl ListBillingFeedbackOption {
    /// Construct a new `ListBillingFeedbackOption`.
    pub fn new() -> Self {
        Self { inner: ListBillingFeedbackOptionBuilder::new() }
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
    /// Filter results to only include feedback options with the given status.
    pub fn status(mut self, status: impl Into<stripe_shared::BillingFeedbackOptionStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListBillingFeedbackOption {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingFeedbackOption {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::BillingFeedbackOption>>
    {
        stripe_client_core::ListPaginator::new_list("/billing/feedback_options", &self.inner)
    }
}

impl StripeRequest for ListBillingFeedbackOption {
    type Output = stripe_types::List<stripe_shared::BillingFeedbackOption>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/feedback_options").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveBillingFeedbackOptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveBillingFeedbackOptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveBillingFeedbackOptionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveBillingFeedbackOptionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a feedback options object given an ID.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveBillingFeedbackOption {
    inner: RetrieveBillingFeedbackOptionBuilder,
    id: stripe_shared::BillingFeedbackOptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveBillingFeedbackOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveBillingFeedbackOption").finish_non_exhaustive()
    }
}
impl RetrieveBillingFeedbackOption {
    /// Construct a new `RetrieveBillingFeedbackOption`.
    pub fn new(id: impl Into<stripe_shared::BillingFeedbackOptionId>) -> Self {
        Self { id: id.into(), inner: RetrieveBillingFeedbackOptionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBillingFeedbackOption {
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

impl StripeRequest for RetrieveBillingFeedbackOption {
    type Output = stripe_shared::BillingFeedbackOption;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/feedback_options/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateBillingFeedbackOptionBuilder {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingFeedbackOptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingFeedbackOptionBuilder").finish_non_exhaustive()
    }
}
impl CreateBillingFeedbackOptionBuilder {
    fn new(description: impl Into<String>) -> Self {
        Self { description: description.into(), expand: None }
    }
}
/// Creates a new feedback option.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingFeedbackOption {
    inner: CreateBillingFeedbackOptionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingFeedbackOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingFeedbackOption").finish_non_exhaustive()
    }
}
impl CreateBillingFeedbackOption {
    /// Construct a new `CreateBillingFeedbackOption`.
    pub fn new(description: impl Into<String>) -> Self {
        Self { inner: CreateBillingFeedbackOptionBuilder::new(description.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateBillingFeedbackOption {
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

impl StripeRequest for CreateBillingFeedbackOption {
    type Output = stripe_shared::BillingFeedbackOption;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/feedback_options").form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateBillingFeedbackOptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingFeedbackOptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingFeedbackOptionBuilder").finish_non_exhaustive()
    }
}
impl UpdateBillingFeedbackOptionBuilder {
    fn new() -> Self {
        Self { description: None, expand: None }
    }
}
/// Updates the description of an existing feedback option.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingFeedbackOption {
    inner: UpdateBillingFeedbackOptionBuilder,
    id: stripe_shared::BillingFeedbackOptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingFeedbackOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingFeedbackOption").finish_non_exhaustive()
    }
}
impl UpdateBillingFeedbackOption {
    /// Construct a new `UpdateBillingFeedbackOption`.
    pub fn new(id: impl Into<stripe_shared::BillingFeedbackOptionId>) -> Self {
        Self { id: id.into(), inner: UpdateBillingFeedbackOptionBuilder::new() }
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UpdateBillingFeedbackOption {
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

impl StripeRequest for UpdateBillingFeedbackOption {
    type Output = stripe_shared::BillingFeedbackOption;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/feedback_options/{id}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct DeactivateBillingFeedbackOptionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeactivateBillingFeedbackOptionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeactivateBillingFeedbackOptionBuilder").finish_non_exhaustive()
    }
}
impl DeactivateBillingFeedbackOptionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Deactivates a feedback option.
/// Deactivated feedback options cannot be used in portal configurations.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeactivateBillingFeedbackOption {
    inner: DeactivateBillingFeedbackOptionBuilder,
    id: stripe_shared::BillingFeedbackOptionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeactivateBillingFeedbackOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeactivateBillingFeedbackOption").finish_non_exhaustive()
    }
}
impl DeactivateBillingFeedbackOption {
    /// Construct a new `DeactivateBillingFeedbackOption`.
    pub fn new(id: impl Into<stripe_shared::BillingFeedbackOptionId>) -> Self {
        Self { id: id.into(), inner: DeactivateBillingFeedbackOptionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeactivateBillingFeedbackOption {
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

impl StripeRequest for DeactivateBillingFeedbackOption {
    type Output = stripe_shared::BillingFeedbackOption;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/billing/feedback_options/{id}/deactivate"),
        )
        .form(&self.inner)
    }
}
