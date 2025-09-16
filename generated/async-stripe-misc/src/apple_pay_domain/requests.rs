use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete an apple pay domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteApplePayDomain {
    domain: String,
}
impl DeleteApplePayDomain {
    /// Construct a new `DeleteApplePayDomain`.
    pub fn new(domain: impl Into<String>) -> Self {
        Self { domain: domain.into() }
    }
}
impl DeleteApplePayDomain {
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

impl StripeRequest for DeleteApplePayDomain {
    type Output = stripe_misc::DeletedApplePayDomain;

    fn build(&self) -> RequestBuilder {
        let domain = &self.domain;
        RequestBuilder::new(StripeMethod::Delete, format!("/apple_pay/domains/{domain}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListApplePayDomainBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListApplePayDomainBuilder {
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
pub struct ListApplePayDomain {
    inner: ListApplePayDomainBuilder,
}
impl ListApplePayDomain {
    /// Construct a new `ListApplePayDomain`.
    pub fn new() -> Self {
        Self { inner: ListApplePayDomainBuilder::new() }
    }
    pub fn domain_name(mut self, domain_name: impl Into<String>) -> Self {
        self.inner.domain_name = Some(domain_name.into());
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
impl Default for ListApplePayDomain {
    fn default() -> Self {
        Self::new()
    }
}
impl ListApplePayDomain {
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
        stripe_client_core::ListPaginator::new_list("/apple_pay/domains", &self.inner)
    }
}

impl StripeRequest for ListApplePayDomain {
    type Output = stripe_types::List<stripe_misc::ApplePayDomain>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/apple_pay/domains").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveApplePayDomainBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveApplePayDomainBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve an apple pay domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveApplePayDomain {
    inner: RetrieveApplePayDomainBuilder,
    domain: String,
}
impl RetrieveApplePayDomain {
    /// Construct a new `RetrieveApplePayDomain`.
    pub fn new(domain: impl Into<String>) -> Self {
        Self { domain: domain.into(), inner: RetrieveApplePayDomainBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveApplePayDomain {
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

impl StripeRequest for RetrieveApplePayDomain {
    type Output = stripe_misc::ApplePayDomain;

    fn build(&self) -> RequestBuilder {
        let domain = &self.domain;
        RequestBuilder::new(StripeMethod::Get, format!("/apple_pay/domains/{domain}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateApplePayDomainBuilder {
    domain_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CreateApplePayDomainBuilder {
    fn new(domain_name: impl Into<String>) -> Self {
        Self { domain_name: domain_name.into(), expand: None }
    }
}
/// Create an apple pay domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateApplePayDomain {
    inner: CreateApplePayDomainBuilder,
}
impl CreateApplePayDomain {
    /// Construct a new `CreateApplePayDomain`.
    pub fn new(domain_name: impl Into<String>) -> Self {
        Self { inner: CreateApplePayDomainBuilder::new(domain_name.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateApplePayDomain {
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

impl StripeRequest for CreateApplePayDomain {
    type Output = stripe_misc::ApplePayDomain;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/apple_pay/domains").form(&self.inner)
    }
}
