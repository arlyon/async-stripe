use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListFileLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListFileLinkBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            expired: None,
            file: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of file links.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFileLink<'a> {
    inner: ListFileLinkBuilder<'a>,
}
impl<'a> ListFileLink<'a> {
    /// Construct a new `ListFileLink`.
    pub fn new() -> Self {
        Self { inner: ListFileLinkBuilder::new() }
    }
    /// Only return links that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Filter links by their expiration status. By default, Stripe returns all links.
    pub fn expired(mut self, expired: bool) -> Self {
        self.inner.expired = Some(expired);
        self
    }
    /// Only return links for the given file.
    pub fn file(mut self, file: &'a str) -> Self {
        self.inner.file = Some(file);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListFileLink<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListFileLink<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::FileLink>> {
        stripe_client_core::ListPaginator::new_list("/file_links", self.inner)
    }
}

impl StripeRequest for ListFileLink<'_> {
    type Output = stripe_types::List<stripe_shared::FileLink>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/file_links").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveFileLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFileLinkBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the file link with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFileLink<'a> {
    inner: RetrieveFileLinkBuilder<'a>,
    link: &'a stripe_shared::FileLinkId,
}
impl<'a> RetrieveFileLink<'a> {
    /// Construct a new `RetrieveFileLink`.
    pub fn new(link: &'a stripe_shared::FileLinkId) -> Self {
        Self { link, inner: RetrieveFileLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveFileLink<'_> {
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

impl StripeRequest for RetrieveFileLink<'_> {
    type Output = stripe_shared::FileLink;

    fn build(&self) -> RequestBuilder {
        let link = self.link;
        RequestBuilder::new(StripeMethod::Get, format!("/file_links/{link}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateFileLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    file: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateFileLinkBuilder<'a> {
    fn new(file: &'a str) -> Self {
        Self { expand: None, expires_at: None, file, metadata: None }
    }
}
/// Creates a new file link object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateFileLink<'a> {
    inner: CreateFileLinkBuilder<'a>,
}
impl<'a> CreateFileLink<'a> {
    /// Construct a new `CreateFileLink`.
    pub fn new(file: &'a str) -> Self {
        Self { inner: CreateFileLinkBuilder::new(file) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The link isn't usable after this future timestamp.
    pub fn expires_at(mut self, expires_at: stripe_types::Timestamp) -> Self {
        self.inner.expires_at = Some(expires_at);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl CreateFileLink<'_> {
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

impl StripeRequest for CreateFileLink<'_> {
    type Output = stripe_shared::FileLink;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/file_links").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateFileLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<UpdateFileLinkExpiresAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateFileLinkBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, expires_at: None, metadata: None }
    }
}
/// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateFileLinkExpiresAt {
    Now,
    Timestamp(stripe_types::Timestamp),
}
/// Updates an existing file link object. Expired links can no longer be updated.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateFileLink<'a> {
    inner: UpdateFileLinkBuilder<'a>,
    link: &'a stripe_shared::FileLinkId,
}
impl<'a> UpdateFileLink<'a> {
    /// Construct a new `UpdateFileLink`.
    pub fn new(link: &'a stripe_shared::FileLinkId) -> Self {
        Self { link, inner: UpdateFileLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
    pub fn expires_at(mut self, expires_at: UpdateFileLinkExpiresAt) -> Self {
        self.inner.expires_at = Some(expires_at);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl UpdateFileLink<'_> {
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

impl StripeRequest for UpdateFileLink<'_> {
    type Output = stripe_shared::FileLink;

    fn build(&self) -> RequestBuilder {
        let link = self.link;
        RequestBuilder::new(StripeMethod::Post, format!("/file_links/{link}")).form(&self.inner)
    }
}
