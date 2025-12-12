use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListFileLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListFileLinkBuilder {
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
pub struct ListFileLink {
    inner: ListFileLinkBuilder,
}
impl ListFileLink {
    /// Construct a new `ListFileLink`.
    pub fn new() -> Self {
        Self { inner: ListFileLinkBuilder::new() }
    }
    /// Only return links that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
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
    /// Filter links by their expiration status. By default, Stripe returns all links.
    pub fn expired(mut self, expired: impl Into<bool>) -> Self {
        self.inner.expired = Some(expired.into());
        self
    }
    /// Only return links for the given file.
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.inner.file = Some(file.into());
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
impl Default for ListFileLink {
    fn default() -> Self {
        Self::new()
    }
}
impl ListFileLink {
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
        stripe_client_core::ListPaginator::new_list("/file_links", &self.inner)
    }
}

impl StripeRequest for ListFileLink {
    type Output = stripe_types::List<stripe_shared::FileLink>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/file_links").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveFileLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveFileLinkBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the file link with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFileLink {
    inner: RetrieveFileLinkBuilder,
    link: stripe_shared::FileLinkId,
}
impl RetrieveFileLink {
    /// Construct a new `RetrieveFileLink`.
    pub fn new(link: impl Into<stripe_shared::FileLinkId>) -> Self {
        Self { link: link.into(), inner: RetrieveFileLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveFileLink {
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

impl StripeRequest for RetrieveFileLink {
    type Output = stripe_shared::FileLink;

    fn build(&self) -> RequestBuilder {
        let link = &self.link;
        RequestBuilder::new(StripeMethod::Get, format!("/file_links/{link}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateFileLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl CreateFileLinkBuilder {
    fn new(file: impl Into<String>) -> Self {
        Self { expand: None, expires_at: None, file: file.into(), metadata: None }
    }
}
/// Creates a new file link object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateFileLink {
    inner: CreateFileLinkBuilder,
}
impl CreateFileLink {
    /// Construct a new `CreateFileLink`.
    pub fn new(file: impl Into<String>) -> Self {
        Self { inner: CreateFileLinkBuilder::new(file.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The link isn't usable after this future timestamp.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
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
impl CreateFileLink {
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

impl StripeRequest for CreateFileLink {
    type Output = stripe_shared::FileLink;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/file_links").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateFileLinkBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<UpdateFileLinkExpiresAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateFileLinkBuilder {
    fn new() -> Self {
        Self { expand: None, expires_at: None, metadata: None }
    }
}
/// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateFileLinkExpiresAt {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Updates an existing file link object. Expired links can no longer be updated.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateFileLink {
    inner: UpdateFileLinkBuilder,
    link: stripe_shared::FileLinkId,
}
impl UpdateFileLink {
    /// Construct a new `UpdateFileLink`.
    pub fn new(link: impl Into<stripe_shared::FileLinkId>) -> Self {
        Self { link: link.into(), inner: UpdateFileLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
    pub fn expires_at(mut self, expires_at: impl Into<UpdateFileLinkExpiresAt>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
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
impl UpdateFileLink {
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

impl StripeRequest for UpdateFileLink {
    type Output = stripe_shared::FileLink;

    fn build(&self) -> RequestBuilder {
        let link = &self.link;
        RequestBuilder::new(StripeMethod::Post, format!("/file_links/{link}")).form(&self.inner)
    }
}
