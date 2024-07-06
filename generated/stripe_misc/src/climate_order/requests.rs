use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListClimateOrderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListClimateOrderBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Lists all Climate order objects. The orders are returned sorted by creation date, with the
/// most recently created orders appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListClimateOrder<'a> {
    inner: ListClimateOrderBuilder<'a>,
}
impl<'a> ListClimateOrder<'a> {
    /// Construct a new `ListClimateOrder`.
    pub fn new() -> Self {
        Self { inner: ListClimateOrderBuilder::new() }
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
impl<'a> Default for ListClimateOrder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListClimateOrder<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ClimateOrder>> {
        stripe_client_core::ListPaginator::new_list("/climate/orders", self.inner)
    }
}

impl StripeRequest for ListClimateOrder<'_> {
    type Output = stripe_types::List<stripe_misc::ClimateOrder>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/climate/orders").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveClimateOrderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveClimateOrderBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a Climate order object with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveClimateOrder<'a> {
    inner: RetrieveClimateOrderBuilder<'a>,
    order: &'a stripe_misc::ClimateOrderId,
}
impl<'a> RetrieveClimateOrder<'a> {
    /// Construct a new `RetrieveClimateOrder`.
    pub fn new(order: &'a stripe_misc::ClimateOrderId) -> Self {
        Self { order, inner: RetrieveClimateOrderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveClimateOrder<'_> {
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

impl StripeRequest for RetrieveClimateOrder<'_> {
    type Output = stripe_misc::ClimateOrder;

    fn build(&self) -> RequestBuilder {
        let order = self.order;
        RequestBuilder::new(StripeMethod::Get, format!("/climate/orders/{order}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateClimateOrderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    beneficiary: Option<BeneficiaryParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_tons: Option<&'a str>,
    product: &'a str,
}
impl<'a> CreateClimateOrderBuilder<'a> {
    fn new(product: &'a str) -> Self {
        Self {
            amount: None,
            beneficiary: None,
            currency: None,
            expand: None,
            metadata: None,
            metric_tons: None,
            product,
        }
    }
}
/// Creates a Climate order object for a given Climate product. The order will be processed immediately
/// after creation and payment will be deducted your Stripe balance.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateClimateOrder<'a> {
    inner: CreateClimateOrderBuilder<'a>,
}
impl<'a> CreateClimateOrder<'a> {
    /// Construct a new `CreateClimateOrder`.
    pub fn new(product: &'a str) -> Self {
        Self { inner: CreateClimateOrderBuilder::new(product) }
    }
    /// Requested amount of carbon removal units. Either this or `metric_tons` must be specified.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// Publicly sharable reference for the end beneficiary of carbon removal.
    /// Assumed to be the Stripe account if not set.
    pub fn beneficiary(mut self, beneficiary: BeneficiaryParams<'a>) -> Self {
        self.inner.beneficiary = Some(beneficiary);
        self
    }
    /// Request currency for the order as a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a supported [settlement currency for your account](https://stripe.com/docs/currencies).
    /// If omitted, the account's default currency will be used.
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
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
    /// Requested number of tons for the order. Either this or `amount` must be specified.
    pub fn metric_tons(mut self, metric_tons: &'a str) -> Self {
        self.inner.metric_tons = Some(metric_tons);
        self
    }
}
impl CreateClimateOrder<'_> {
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

impl StripeRequest for CreateClimateOrder<'_> {
    type Output = stripe_misc::ClimateOrder;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/climate/orders").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateClimateOrderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    beneficiary: Option<BeneficiaryParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateClimateOrderBuilder<'a> {
    fn new() -> Self {
        Self { beneficiary: None, expand: None, metadata: None }
    }
}
/// Updates the specified order by setting the values of the parameters passed.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateClimateOrder<'a> {
    inner: UpdateClimateOrderBuilder<'a>,
    order: &'a stripe_misc::ClimateOrderId,
}
impl<'a> UpdateClimateOrder<'a> {
    /// Construct a new `UpdateClimateOrder`.
    pub fn new(order: &'a stripe_misc::ClimateOrderId) -> Self {
        Self { order, inner: UpdateClimateOrderBuilder::new() }
    }
    /// Publicly sharable reference for the end beneficiary of carbon removal.
    /// Assumed to be the Stripe account if not set.
    pub fn beneficiary(mut self, beneficiary: BeneficiaryParams<'a>) -> Self {
        self.inner.beneficiary = Some(beneficiary);
        self
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
}
impl UpdateClimateOrder<'_> {
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

impl StripeRequest for UpdateClimateOrder<'_> {
    type Output = stripe_misc::ClimateOrder;

    fn build(&self) -> RequestBuilder {
        let order = self.order;
        RequestBuilder::new(StripeMethod::Post, format!("/climate/orders/{order}"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelClimateOrderBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelClimateOrderBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels a Climate order. You can cancel an order within 30 days of creation. Stripe refunds the
/// reservation `amount_subtotal`, but not the `amount_fees` for user-triggered cancellations. Frontier
/// might cancel reservations if suppliers fail to deliver. If Frontier cancels the reservation, Stripe
/// provides 90 days advance notice and refunds the `amount_total`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelClimateOrder<'a> {
    inner: CancelClimateOrderBuilder<'a>,
    order: &'a stripe_misc::ClimateOrderId,
}
impl<'a> CancelClimateOrder<'a> {
    /// Construct a new `CancelClimateOrder`.
    pub fn new(order: &'a stripe_misc::ClimateOrderId) -> Self {
        Self { order, inner: CancelClimateOrderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelClimateOrder<'_> {
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

impl StripeRequest for CancelClimateOrder<'_> {
    type Output = stripe_misc::ClimateOrder;

    fn build(&self) -> RequestBuilder {
        let order = self.order;
        RequestBuilder::new(StripeMethod::Post, format!("/climate/orders/{order}/cancel"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct BeneficiaryParams<'a> {
    /// Publicly displayable name for the end beneficiary of carbon removal.
    pub public_name: &'a str,
}
impl<'a> BeneficiaryParams<'a> {
    pub fn new(public_name: &'a str) -> Self {
        Self { public_name }
    }
}
