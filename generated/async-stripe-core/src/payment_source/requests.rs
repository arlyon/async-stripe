use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListCustomerPaymentSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListCustomerPaymentSourceBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, object: None, starting_after: None }
    }
}
/// List sources for a specified customer.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListCustomerPaymentSource {
    inner: ListCustomerPaymentSourceBuilder,
    customer: stripe_shared::CustomerId,
}
impl ListCustomerPaymentSource {
    /// Construct a new `ListCustomerPaymentSource`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>) -> Self {
        Self { customer: customer.into(), inner: ListCustomerPaymentSourceBuilder::new() }
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
    /// Filter sources according to a particular object type.
    pub fn object(mut self, object: impl Into<String>) -> Self {
        self.inner.object = Some(object.into());
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
impl ListCustomerPaymentSource {
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
        let customer = &self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/sources"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListCustomerPaymentSource {
    type Output = stripe_types::List<stripe_shared::PaymentSource>;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/sources"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrievePaymentSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePaymentSourceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrievePaymentSource {
    inner: RetrievePaymentSourceBuilder,
    customer: stripe_shared::CustomerId,
    id: String,
}
impl RetrievePaymentSource {
    /// Construct a new `RetrievePaymentSource`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            id: id.into(),
            inner: RetrievePaymentSourceBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePaymentSource {
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

impl StripeRequest for RetrievePaymentSource {
    type Output = stripe_shared::PaymentSource;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/sources/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateCustomerPaymentSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<bool>,
}
impl CreateCustomerPaymentSourceBuilder {
    fn new(source: impl Into<String>) -> Self {
        Self { expand: None, metadata: None, source: source.into(), validate: None }
    }
}
/// When you create a new credit card, you must specify a customer or recipient on which to create it.
///
/// If the card’s owner has no default card, then the new card will become the default.
/// However, if the owner already has a default, then it will not change.
/// To change the default, you should [update the customer](https://stripe.com/docs/api#update_customer) to have a new `default_source`.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateCustomerPaymentSource {
    inner: CreateCustomerPaymentSourceBuilder,
    customer: stripe_shared::CustomerId,
}
impl CreateCustomerPaymentSource {
    /// Construct a new `CreateCustomerPaymentSource`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, source: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            inner: CreateCustomerPaymentSourceBuilder::new(source.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    pub fn validate(mut self, validate: impl Into<bool>) -> Self {
        self.inner.validate = Some(validate.into());
        self
    }
}
impl CreateCustomerPaymentSource {
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

impl StripeRequest for CreateCustomerPaymentSource {
    type Output = stripe_shared::PaymentSource;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        RequestBuilder::new(StripeMethod::Post, format!("/customers/{customer}/sources"))
            .form(&self.inner)
    }
}
