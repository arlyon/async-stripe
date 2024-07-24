use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListReviewBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListReviewBuilder<'a> {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of `Review` objects that have `open` set to `true`.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListReview<'a> {
    inner: ListReviewBuilder<'a>,
}
impl<'a> ListReview<'a> {
    /// Construct a new `ListReview`.
    pub fn new() -> Self {
        Self { inner: ListReviewBuilder::new() }
    }
    /// Only return reviews that were created during the given date interval.
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListReview<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListReview<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Review>> {
        stripe_client_core::ListPaginator::new_list("/reviews", self.inner)
    }
}

impl StripeRequest for ListReview<'_> {
    type Output = stripe_types::List<stripe_shared::Review>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/reviews").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveReviewBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReviewBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Review` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveReview<'a> {
    inner: RetrieveReviewBuilder<'a>,
    review: &'a stripe_shared::ReviewId,
}
impl<'a> RetrieveReview<'a> {
    /// Construct a new `RetrieveReview`.
    pub fn new(review: &'a stripe_shared::ReviewId) -> Self {
        Self { review, inner: RetrieveReviewBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveReview<'_> {
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

impl StripeRequest for RetrieveReview<'_> {
    type Output = stripe_shared::Review;

    fn build(&self) -> RequestBuilder {
        let review = self.review;
        RequestBuilder::new(StripeMethod::Get, format!("/reviews/{review}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ApproveReviewBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ApproveReviewBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Approves a `Review` object, closing it and removing it from the list of reviews.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ApproveReview<'a> {
    inner: ApproveReviewBuilder<'a>,
    review: &'a stripe_shared::ReviewId,
}
impl<'a> ApproveReview<'a> {
    /// Construct a new `ApproveReview`.
    pub fn new(review: &'a stripe_shared::ReviewId) -> Self {
        Self { review, inner: ApproveReviewBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl ApproveReview<'_> {
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

impl StripeRequest for ApproveReview<'_> {
    type Output = stripe_shared::Review;

    fn build(&self) -> RequestBuilder {
        let review = self.review;
        RequestBuilder::new(StripeMethod::Post, format!("/reviews/{review}/approve"))
            .form(&self.inner)
    }
}
