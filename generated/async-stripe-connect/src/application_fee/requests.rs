use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListApplicationFeeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<&'a str>,
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
impl<'a> ListApplicationFeeBuilder<'a> {
    fn new() -> Self {
        Self {
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of application fees you’ve previously collected.
/// The application fees are returned in sorted order, with the most recent fees appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListApplicationFee<'a> {
    inner: ListApplicationFeeBuilder<'a>,
}
impl<'a> ListApplicationFee<'a> {
    /// Construct a new `ListApplicationFee`.
    pub fn new() -> Self {
        Self { inner: ListApplicationFeeBuilder::new() }
    }
    /// Only return application fees for the charge specified by this charge ID.
    pub fn charge(mut self, charge: &'a str) -> Self {
        self.inner.charge = Some(charge);
        self
    }
    /// Only return applications fees that were created during the given date interval.
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
impl<'a> Default for ListApplicationFee<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListApplicationFee<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::ApplicationFee>> {
        stripe_client_core::ListPaginator::new_list("/application_fees", self.inner)
    }
}

impl StripeRequest for ListApplicationFee<'_> {
    type Output = stripe_types::List<stripe_shared::ApplicationFee>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/application_fees").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveApplicationFeeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveApplicationFeeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an application fee that your account has collected.
/// The same information is returned when refunding the application fee.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveApplicationFee<'a> {
    inner: RetrieveApplicationFeeBuilder<'a>,
    id: &'a stripe_shared::ApplicationFeeId,
}
impl<'a> RetrieveApplicationFee<'a> {
    /// Construct a new `RetrieveApplicationFee`.
    pub fn new(id: &'a stripe_shared::ApplicationFeeId) -> Self {
        Self { id, inner: RetrieveApplicationFeeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveApplicationFee<'_> {
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

impl StripeRequest for RetrieveApplicationFee<'_> {
    type Output = stripe_shared::ApplicationFee;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/application_fees/{id}")).query(&self.inner)
    }
}
