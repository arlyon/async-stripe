use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCountrySpecBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListCountrySpecBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Lists all Country Spec objects available in the API.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCountrySpec<'a> {
    inner: ListCountrySpecBuilder<'a>,
}
impl<'a> ListCountrySpec<'a> {
    /// Construct a new `ListCountrySpec`.
    pub fn new() -> Self {
        Self { inner: ListCountrySpecBuilder::new() }
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
impl<'a> Default for ListCountrySpec<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCountrySpec<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_connect::CountrySpec>> {
        stripe_client_core::ListPaginator::new_list("/country_specs", self.inner)
    }
}

impl StripeRequest for ListCountrySpec<'_> {
    type Output = stripe_types::List<stripe_connect::CountrySpec>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/country_specs").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveCountrySpecBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCountrySpecBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a Country Spec for a given Country code.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCountrySpec<'a> {
    inner: RetrieveCountrySpecBuilder<'a>,
    country: &'a stripe_connect::CountrySpecId,
}
impl<'a> RetrieveCountrySpec<'a> {
    /// Construct a new `RetrieveCountrySpec`.
    pub fn new(country: &'a stripe_connect::CountrySpecId) -> Self {
        Self { country, inner: RetrieveCountrySpecBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCountrySpec<'_> {
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

impl StripeRequest for RetrieveCountrySpec<'_> {
    type Output = stripe_connect::CountrySpec;

    fn build(&self) -> RequestBuilder {
        let country = self.country;
        RequestBuilder::new(StripeMethod::Get, format!("/country_specs/{country}"))
            .query(&self.inner)
    }
}
