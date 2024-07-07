use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListEntitlementsActiveEntitlementBuilder<'a> {
    customer: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListEntitlementsActiveEntitlementBuilder<'a> {
    fn new(customer: &'a str) -> Self {
        Self { customer, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieve a list of active entitlements for a customer
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListEntitlementsActiveEntitlement<'a> {
    inner: ListEntitlementsActiveEntitlementBuilder<'a>,
}
impl<'a> ListEntitlementsActiveEntitlement<'a> {
    /// Construct a new `ListEntitlementsActiveEntitlement`.
    pub fn new(customer: &'a str) -> Self {
        Self { inner: ListEntitlementsActiveEntitlementBuilder::new(customer) }
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
impl ListEntitlementsActiveEntitlement<'_> {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_misc::EntitlementsActiveEntitlement>,
    > {
        stripe_client_core::ListPaginator::new_list("/entitlements/active_entitlements", self.inner)
    }
}

impl StripeRequest for ListEntitlementsActiveEntitlement<'_> {
    type Output = stripe_types::List<stripe_misc::EntitlementsActiveEntitlement>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/entitlements/active_entitlements")
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveEntitlementsActiveEntitlementBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveEntitlementsActiveEntitlementBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve an active entitlement
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveEntitlementsActiveEntitlement<'a> {
    inner: RetrieveEntitlementsActiveEntitlementBuilder<'a>,
    id: &'a stripe_misc::EntitlementsActiveEntitlementId,
}
impl<'a> RetrieveEntitlementsActiveEntitlement<'a> {
    /// Construct a new `RetrieveEntitlementsActiveEntitlement`.
    pub fn new(id: &'a stripe_misc::EntitlementsActiveEntitlementId) -> Self {
        Self { id, inner: RetrieveEntitlementsActiveEntitlementBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveEntitlementsActiveEntitlement<'_> {
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

impl StripeRequest for RetrieveEntitlementsActiveEntitlement<'_> {
    type Output = stripe_misc::EntitlementsActiveEntitlement;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/entitlements/active_entitlements/{id}"))
            .query(&self.inner)
    }
}
