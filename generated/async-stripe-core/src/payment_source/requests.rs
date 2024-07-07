use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCustomerPaymentSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListCustomerPaymentSourceBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, object: None, starting_after: None }
    }
}
/// List sources for a specified customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCustomerPaymentSource<'a> {
    inner: ListCustomerPaymentSourceBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> ListCustomerPaymentSource<'a> {
    /// Construct a new `ListCustomerPaymentSource`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: ListCustomerPaymentSourceBuilder::new() }
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
    /// Filter sources according to a particular object type.
    pub fn object(mut self, object: &'a str) -> Self {
        self.inner.object = Some(object);
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
impl ListCustomerPaymentSource<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::PaymentSource>> {
        let customer = self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/sources"),
            self.inner,
        )
    }
}

impl StripeRequest for ListCustomerPaymentSource<'_> {
    type Output = stripe_types::List<stripe_shared::PaymentSource>;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/sources"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePaymentSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentSourceBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentSource<'a> {
    inner: RetrievePaymentSourceBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    id: &'a str,
}
impl<'a> RetrievePaymentSource<'a> {
    /// Construct a new `RetrievePaymentSource`.
    pub fn new(customer: &'a stripe_shared::CustomerId, id: &'a str) -> Self {
        Self { customer, id, inner: RetrievePaymentSourceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePaymentSource<'_> {
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

impl StripeRequest for RetrievePaymentSource<'_> {
    type Output = stripe_shared::PaymentSource;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/sources/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateCustomerPaymentSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    source: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<bool>,
}
impl<'a> CreateCustomerPaymentSourceBuilder<'a> {
    fn new(source: &'a str) -> Self {
        Self { expand: None, metadata: None, source, validate: None }
    }
}
/// When you create a new credit card, you must specify a customer or recipient on which to create it.
///
/// If the card’s owner has no default card, then the new card will become the default.
/// However, if the owner already has a default, then it will not change.
/// To change the default, you should [update the customer](https://stripe.com/docs/api#update_customer) to have a new `default_source`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerPaymentSource<'a> {
    inner: CreateCustomerPaymentSourceBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> CreateCustomerPaymentSource<'a> {
    /// Construct a new `CreateCustomerPaymentSource`.
    pub fn new(customer: &'a stripe_shared::CustomerId, source: &'a str) -> Self {
        Self { customer, inner: CreateCustomerPaymentSourceBuilder::new(source) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    pub fn validate(mut self, validate: bool) -> Self {
        self.inner.validate = Some(validate);
        self
    }
}
impl CreateCustomerPaymentSource<'_> {
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

impl StripeRequest for CreateCustomerPaymentSource<'_> {
    type Output = stripe_shared::PaymentSource;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Post, format!("/customers/{customer}/sources"))
            .form(&self.inner)
    }
}
