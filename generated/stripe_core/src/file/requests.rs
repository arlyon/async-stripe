use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListFileBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<stripe_shared::FilePurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListFileBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            purpose: None,
            starting_after: None,
        }
    }
}
/// Returns a list of the files that your account has access to.
/// Stripe sorts and returns the files by their creation dates, placing the most recently created files at the top.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFile<'a> {
    inner: ListFileBuilder<'a>,
}
impl<'a> ListFile<'a> {
    /// Construct a new `ListFile`.
    pub fn new() -> Self {
        Self { inner: ListFileBuilder::new() }
    }
    /// Only return files that were created during the given date interval.
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
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Filter queries by the file purpose.
    /// If you don't provide a purpose, the queries return unfiltered files.
    pub fn purpose(mut self, purpose: stripe_shared::FilePurpose) -> Self {
        self.inner.purpose = Some(purpose);
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
impl<'a> Default for ListFile<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListFile<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::File>> {
        stripe_client_core::ListPaginator::new_list("/files", self.inner)
    }
}

impl StripeRequest for ListFile<'_> {
    type Output = stripe_types::List<stripe_shared::File>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/files").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveFileBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFileBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing file object.
/// After you supply a unique file ID, Stripe returns the corresponding file object.
/// Learn how to [access file contents](https://stripe.com/docs/file-upload#download-file-contents).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFile<'a> {
    inner: RetrieveFileBuilder<'a>,
    file: &'a stripe_shared::FileId,
}
impl<'a> RetrieveFile<'a> {
    /// Construct a new `RetrieveFile`.
    pub fn new(file: &'a stripe_shared::FileId) -> Self {
        Self { file, inner: RetrieveFileBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveFile<'_> {
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

impl StripeRequest for RetrieveFile<'_> {
    type Output = stripe_shared::File;

    fn build(&self) -> RequestBuilder {
        let file = self.file;
        RequestBuilder::new(StripeMethod::Get, format!("/files/{file}")).query(&self.inner)
    }
}
