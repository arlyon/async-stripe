use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListForwardingRequestBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<ListForwardingRequestCreated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListForwardingRequestBuilder<'a> {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Similar to other List endpoints, filters results based on created timestamp.
/// You can pass gt, gte, lt, and lte timestamp values.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
pub struct ListForwardingRequest<'a> {
    inner: ListForwardingRequestBuilder<'a>,
}
impl<'a> ListForwardingRequest<'a> {
    /// Construct a new `ListForwardingRequest`.
    pub fn new() -> Self {
        Self { inner: ListForwardingRequestBuilder::new() }
    }
    /// Similar to other List endpoints, filters results based on created timestamp.
    /// You can pass gt, gte, lt, and lte timestamp values.
    pub fn created(mut self, created: ListForwardingRequestCreated) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// A pagination cursor to fetch the previous page of the list.
    /// The value must be a ForwardingRequest ID.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A pagination cursor to fetch the next page of the list. The value must be a ForwardingRequest ID.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListForwardingRequest<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListForwardingRequest<'_> {
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
        stripe_client_core::ListPaginator::new_list("/forwarding/requests", self.inner)
    }
}

impl StripeRequest for ListForwardingRequest<'_> {
    type Output = stripe_types::List<stripe_misc::ForwardingRequest>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/forwarding/requests").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveForwardingRequestBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveForwardingRequestBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a ForwardingRequest object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForwardingRequest<'a> {
    inner: RetrieveForwardingRequestBuilder<'a>,
    id: &'a stripe_misc::ForwardingRequestId,
}
impl<'a> RetrieveForwardingRequest<'a> {
    /// Construct a new `RetrieveForwardingRequest`.
    pub fn new(id: &'a stripe_misc::ForwardingRequestId) -> Self {
        Self { id, inner: RetrieveForwardingRequestBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveForwardingRequest<'_> {
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

impl StripeRequest for RetrieveForwardingRequest<'_> {
    type Output = stripe_misc::ForwardingRequest;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/forwarding/requests/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateForwardingRequestBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    payment_method: &'a str,
    replacements: &'a [stripe_misc::ForwardingRequestReplacements],
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<CreateForwardingRequestRequest<'a>>,
    url: &'a str,
}
impl<'a> CreateForwardingRequestBuilder<'a> {
    fn new(
        payment_method: &'a str,
        replacements: &'a [stripe_misc::ForwardingRequestReplacements],
        url: &'a str,
    ) -> Self {
        Self { expand: None, payment_method, replacements, request: None, url }
    }
}
/// The request body and headers to be sent to the destination endpoint.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateForwardingRequestRequest<'a> {
    /// The body payload to send to the destination endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<&'a str>,
    /// The headers to include in the forwarded request.
    /// Can be omitted if no additional headers (excluding Stripe-generated ones such as the Content-Type header) should be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<&'a [CreateForwardingRequestRequestHeaders<'a>]>,
}
impl<'a> CreateForwardingRequestRequest<'a> {
    pub fn new() -> Self {
        Self { body: None, headers: None }
    }
}
impl<'a> Default for CreateForwardingRequestRequest<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The headers to include in the forwarded request.
/// Can be omitted if no additional headers (excluding Stripe-generated ones such as the Content-Type header) should be included.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateForwardingRequestRequestHeaders<'a> {
    /// The header name.
    pub name: &'a str,
    /// The header value.
    pub value: &'a str,
}
impl<'a> CreateForwardingRequestRequestHeaders<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
/// Creates a ForwardingRequest object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateForwardingRequest<'a> {
    inner: CreateForwardingRequestBuilder<'a>,
}
impl<'a> CreateForwardingRequest<'a> {
    /// Construct a new `CreateForwardingRequest`.
    pub fn new(
        payment_method: &'a str,
        replacements: &'a [stripe_misc::ForwardingRequestReplacements],
        url: &'a str,
    ) -> Self {
        Self { inner: CreateForwardingRequestBuilder::new(payment_method, replacements, url) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The request body and headers to be sent to the destination endpoint.
    pub fn request(mut self, request: CreateForwardingRequestRequest<'a>) -> Self {
        self.inner.request = Some(request);
        self
    }
}
impl CreateForwardingRequest<'_> {
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

impl StripeRequest for CreateForwardingRequest<'_> {
    type Output = stripe_misc::ForwardingRequest;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/forwarding/requests").form(&self.inner)
    }
}
