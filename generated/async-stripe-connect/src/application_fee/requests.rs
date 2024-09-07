use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListApplicationFeeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListApplicationFeeBuilder {
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
pub struct ListApplicationFee {
    inner: ListApplicationFeeBuilder,
}
impl ListApplicationFee {
    /// Construct a new `ListApplicationFee`.
    pub fn new() -> Self {
        Self { inner: ListApplicationFeeBuilder::new() }
    }
    /// Only return application fees for the charge specified by this charge ID.
    pub fn charge(mut self, charge: impl Into<String>) -> Self {
        self.inner.charge = Some(charge.into());
        self
    }
    /// Only return applications fees that were created during the given date interval.
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
impl Default for ListApplicationFee {
    fn default() -> Self {
        Self::new()
    }
}
impl ListApplicationFee {
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
        stripe_client_core::ListPaginator::new_list("/application_fees", &self.inner)
    }
}

impl StripeRequest for ListApplicationFee {
    type Output = stripe_types::List<stripe_shared::ApplicationFee>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/application_fees").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveApplicationFeeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveApplicationFeeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an application fee that your account has collected.
/// The same information is returned when refunding the application fee.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveApplicationFee {
    inner: RetrieveApplicationFeeBuilder,
    id: stripe_shared::ApplicationFeeId,
}
impl RetrieveApplicationFee {
    /// Construct a new `RetrieveApplicationFee`.
    pub fn new(id: impl Into<stripe_shared::ApplicationFeeId>) -> Self {
        Self { id: id.into(), inner: RetrieveApplicationFeeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveApplicationFee {
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

impl StripeRequest for RetrieveApplicationFee {
    type Output = stripe_shared::ApplicationFee;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/application_fees/{id}")).query(&self.inner)
    }
}
