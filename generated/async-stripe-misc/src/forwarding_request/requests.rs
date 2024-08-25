use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListForwardingRequestBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<ListForwardingRequestCreated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListForwardingRequestBuilder {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Similar to other List endpoints, filters results based on created timestamp.
/// You can pass gt, gte, lt, and lte timestamp values.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListForwardingRequestCreated {
    /// Return results where the `created` field is greater than this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i64>,
    /// Return results where the `created` field is greater than or equal to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i64>,
    /// Return results where the `created` field is less than this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i64>,
    /// Return results where the `created` field is less than or equal to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i64>,
}
impl ListForwardingRequestCreated {
    pub fn new() -> Self {
        Self { gt: None, gte: None, lt: None, lte: None }
    }
}
impl Default for ListForwardingRequestCreated {
    fn default() -> Self {
        Self::new()
    }
}
/// Lists all ForwardingRequest objects.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListForwardingRequest {
    inner: ListForwardingRequestBuilder,
}
impl ListForwardingRequest {
    /// Construct a new `ListForwardingRequest`.
    pub fn new() -> Self {
        Self { inner: ListForwardingRequestBuilder::new() }
    }
    /// Similar to other List endpoints, filters results based on created timestamp.
    /// You can pass gt, gte, lt, and lte timestamp values.
    pub fn created(mut self, created: impl Into<ListForwardingRequestCreated>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// A pagination cursor to fetch the previous page of the list.
    /// The value must be a ForwardingRequest ID.
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
    /// A pagination cursor to fetch the next page of the list. The value must be a ForwardingRequest ID.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListForwardingRequest {
    fn default() -> Self {
        Self::new()
    }
}
impl ListForwardingRequest {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ForwardingRequest>> {
        stripe_client_core::ListPaginator::new_list("/forwarding/requests", &self.inner)
    }
}

impl StripeRequest for ListForwardingRequest {
    type Output = stripe_types::List<stripe_misc::ForwardingRequest>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/forwarding/requests").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveForwardingRequestBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveForwardingRequestBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a ForwardingRequest object.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveForwardingRequest {
    inner: RetrieveForwardingRequestBuilder,
    id: stripe_misc::ForwardingRequestId,
}
impl RetrieveForwardingRequest {
    /// Construct a new `RetrieveForwardingRequest`.
    pub fn new(id: impl Into<stripe_misc::ForwardingRequestId>) -> Self {
        Self { id: id.into(), inner: RetrieveForwardingRequestBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveForwardingRequest {
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

impl StripeRequest for RetrieveForwardingRequest {
    type Output = stripe_misc::ForwardingRequest;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/forwarding/requests/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateForwardingRequestBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    payment_method: String,
    replacements: Vec<stripe_misc::ForwardingRequestReplacements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<CreateForwardingRequestRequest>,
    url: String,
}
impl CreateForwardingRequestBuilder {
    fn new(
        payment_method: impl Into<String>,
        replacements: impl Into<Vec<stripe_misc::ForwardingRequestReplacements>>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            expand: None,
            payment_method: payment_method.into(),
            replacements: replacements.into(),
            request: None,
            url: url.into(),
        }
    }
}
/// The request body and headers to be sent to the destination endpoint.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateForwardingRequestRequest {
    /// The body payload to send to the destination endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The headers to include in the forwarded request.
    /// Can be omitted if no additional headers (excluding Stripe-generated ones such as the Content-Type header) should be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<CreateForwardingRequestRequestHeaders>>,
}
impl CreateForwardingRequestRequest {
    pub fn new() -> Self {
        Self { body: None, headers: None }
    }
}
impl Default for CreateForwardingRequestRequest {
    fn default() -> Self {
        Self::new()
    }
}
/// The headers to include in the forwarded request.
/// Can be omitted if no additional headers (excluding Stripe-generated ones such as the Content-Type header) should be included.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateForwardingRequestRequestHeaders {
    /// The header name.
    pub name: String,
    /// The header value.
    pub value: String,
}
impl CreateForwardingRequestRequestHeaders {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}
/// Creates a ForwardingRequest object.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateForwardingRequest {
    inner: CreateForwardingRequestBuilder,
}
impl CreateForwardingRequest {
    /// Construct a new `CreateForwardingRequest`.
    pub fn new(
        payment_method: impl Into<String>,
        replacements: impl Into<Vec<stripe_misc::ForwardingRequestReplacements>>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateForwardingRequestBuilder::new(
                payment_method.into(),
                replacements.into(),
                url.into(),
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The request body and headers to be sent to the destination endpoint.
    pub fn request(mut self, request: impl Into<CreateForwardingRequestRequest>) -> Self {
        self.inner.request = Some(request.into());
        self
    }
}
impl CreateForwardingRequest {
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

impl StripeRequest for CreateForwardingRequest {
    type Output = stripe_misc::ForwardingRequest;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/forwarding/requests").form(&self.inner)
    }
}
