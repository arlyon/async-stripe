use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete an apple pay domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteApplePayDomain<'a> {
    domain: &'a str,
}
impl<'a> DeleteApplePayDomain<'a> {
    /// Construct a new `DeleteApplePayDomain`.
    pub fn new(domain: &'a str) -> Self {
        Self { domain }
    }
}
impl DeleteApplePayDomain<'_> {
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

impl StripeRequest for DeleteApplePayDomain<'_> {
    type Output = stripe_misc::DeletedApplePayDomain;

    fn build(&self) -> RequestBuilder {
        let domain = self.domain;
        RequestBuilder::new(StripeMethod::Delete, format!("/apple_pay/domains/{domain}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListApplePayDomainBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListApplePayDomainBuilder<'a> {
    fn new() -> Self {
        Self {
            domain_name: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// List apple pay domains.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListApplePayDomain<'a> {
    inner: ListApplePayDomainBuilder<'a>,
}
impl<'a> ListApplePayDomain<'a> {
    /// Construct a new `ListApplePayDomain`.
    pub fn new() -> Self {
        Self { inner: ListApplePayDomainBuilder::new() }
    }
    pub fn domain_name(mut self, domain_name: &'a str) -> Self {
        self.inner.domain_name = Some(domain_name);
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
impl<'a> Default for ListApplePayDomain<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListApplePayDomain<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ApplePayDomain>> {
        stripe_client_core::ListPaginator::new_list("/apple_pay/domains", self.inner)
    }
}

impl StripeRequest for ListApplePayDomain<'_> {
    type Output = stripe_types::List<stripe_misc::ApplePayDomain>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apple_pay/domains").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveApplePayDomainBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveApplePayDomainBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve an apple pay domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveApplePayDomain<'a> {
    inner: RetrieveApplePayDomainBuilder<'a>,
    domain: &'a str,
}
impl<'a> RetrieveApplePayDomain<'a> {
    /// Construct a new `RetrieveApplePayDomain`.
    pub fn new(domain: &'a str) -> Self {
        Self { domain, inner: RetrieveApplePayDomainBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveApplePayDomain<'_> {
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

impl StripeRequest for RetrieveApplePayDomain<'_> {
    type Output = stripe_misc::ApplePayDomain;

    fn build(&self) -> RequestBuilder {
        let domain = self.domain;
        RequestBuilder::new(StripeMethod::Get, format!("/apple_pay/domains/{domain}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateApplePayDomainBuilder<'a> {
    domain_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CreateApplePayDomainBuilder<'a> {
    fn new(domain_name: &'a str) -> Self {
        Self { domain_name, expand: None }
    }
}
/// Create an apple pay domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateApplePayDomain<'a> {
    inner: CreateApplePayDomainBuilder<'a>,
}
impl<'a> CreateApplePayDomain<'a> {
    /// Construct a new `CreateApplePayDomain`.
    pub fn new(domain_name: &'a str) -> Self {
        Self { inner: CreateApplePayDomainBuilder::new(domain_name) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreateApplePayDomain<'_> {
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

impl StripeRequest for CreateApplePayDomain<'_> {
    type Output = stripe_misc::ApplePayDomain;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apple_pay/domains").form(&self.inner)
    }
}
