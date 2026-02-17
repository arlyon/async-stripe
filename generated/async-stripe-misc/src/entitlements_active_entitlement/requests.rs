use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListEntitlementsActiveEntitlementBuilder {
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListEntitlementsActiveEntitlementBuilder {
    fn new(customer: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Retrieve a list of active entitlements for a customer
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListEntitlementsActiveEntitlement {
    inner: ListEntitlementsActiveEntitlementBuilder,
}
impl ListEntitlementsActiveEntitlement {
    /// Construct a new `ListEntitlementsActiveEntitlement`.
    pub fn new(customer: impl Into<String>) -> Self {
        Self { inner: ListEntitlementsActiveEntitlementBuilder::new(customer.into()) }
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
impl ListEntitlementsActiveEntitlement {
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
        stripe_client_core::ListPaginator::new_list(
            "/entitlements/active_entitlements",
            &self.inner,
        )
    }
}

impl StripeRequest for ListEntitlementsActiveEntitlement {
    type Output = stripe_types::List<stripe_misc::EntitlementsActiveEntitlement>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/entitlements/active_entitlements")
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveEntitlementsActiveEntitlementBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveEntitlementsActiveEntitlementBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve an active entitlement
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveEntitlementsActiveEntitlement {
    inner: RetrieveEntitlementsActiveEntitlementBuilder,
    id: stripe_misc::EntitlementsActiveEntitlementId,
}
impl RetrieveEntitlementsActiveEntitlement {
    /// Construct a new `RetrieveEntitlementsActiveEntitlement`.
    pub fn new(id: impl Into<stripe_misc::EntitlementsActiveEntitlementId>) -> Self {
        Self { id: id.into(), inner: RetrieveEntitlementsActiveEntitlementBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveEntitlementsActiveEntitlement {
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

impl StripeRequest for RetrieveEntitlementsActiveEntitlement {
    type Output = stripe_misc::EntitlementsActiveEntitlement;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/entitlements/active_entitlements/{id}"))
            .query(&self.inner)
    }
}
