use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveApplicationFeeRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveApplicationFeeRefundBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// By default, you can see the 10 most recent refunds stored directly on the application fee object, but you can also retrieve details about a specific refund stored on the application fee.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveApplicationFeeRefund<'a> {
    inner: RetrieveApplicationFeeRefundBuilder<'a>,
    fee: &'a stripe_shared::ApplicationFeeId,
    id: &'a str,
}
impl<'a> RetrieveApplicationFeeRefund<'a> {
    /// Construct a new `RetrieveApplicationFeeRefund`.
    pub fn new(fee: &'a stripe_shared::ApplicationFeeId, id: &'a str) -> Self {
        Self { fee, id, inner: RetrieveApplicationFeeRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveApplicationFeeRefund<'_> {
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

impl StripeRequest for RetrieveApplicationFeeRefund<'_> {
    type Output = stripe_shared::ApplicationFeeRefund;

    fn build(&self) -> RequestBuilder {
        let fee = self.fee;
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/application_fees/{fee}/refunds/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIdApplicationFeeRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListIdApplicationFeeRefundBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// You can see a list of the refunds belonging to a specific application fee.
/// Note that the 10 most recent refunds are always available by default on the application fee object.
/// If you need more than those 10, you can use this API method and the `limit` and `starting_after` parameters to page through additional refunds.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIdApplicationFeeRefund<'a> {
    inner: ListIdApplicationFeeRefundBuilder<'a>,
    id: &'a stripe_shared::ApplicationFeeId,
}
impl<'a> ListIdApplicationFeeRefund<'a> {
    /// Construct a new `ListIdApplicationFeeRefund`.
    pub fn new(id: &'a stripe_shared::ApplicationFeeId) -> Self {
        Self { id, inner: ListIdApplicationFeeRefundBuilder::new() }
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
impl ListIdApplicationFeeRefund<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::ApplicationFeeRefund>>
    {
        let id = self.id;

        stripe_client_core::ListPaginator::new_list(
            format!("/application_fees/{id}/refunds"),
            self.inner,
        )
    }
}

impl StripeRequest for ListIdApplicationFeeRefund<'_> {
    type Output = stripe_types::List<stripe_shared::ApplicationFeeRefund>;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/application_fees/{id}/refunds"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateApplicationFeeRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateApplicationFeeRefundBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified application fee refund by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
///
/// This request only accepts metadata as an argument.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateApplicationFeeRefund<'a> {
    inner: UpdateApplicationFeeRefundBuilder<'a>,
    fee: &'a stripe_shared::ApplicationFeeId,
    id: &'a str,
}
impl<'a> UpdateApplicationFeeRefund<'a> {
    /// Construct a new `UpdateApplicationFeeRefund`.
    pub fn new(fee: &'a stripe_shared::ApplicationFeeId, id: &'a str) -> Self {
        Self { fee, id, inner: UpdateApplicationFeeRefundBuilder::new() }
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
impl UpdateApplicationFeeRefund<'_> {
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

impl StripeRequest for UpdateApplicationFeeRefund<'_> {
    type Output = stripe_shared::ApplicationFeeRefund;

    fn build(&self) -> RequestBuilder {
        let fee = self.fee;
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/application_fees/{fee}/refunds/{id}"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateIdApplicationFeeRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateIdApplicationFeeRefundBuilder<'a> {
    fn new() -> Self {
        Self { amount: None, expand: None, metadata: None }
    }
}
/// Refunds an application fee that has previously been collected but not yet refunded.
/// Funds will be refunded to the Stripe account from which the fee was originally collected.
///
/// You can optionally refund only part of an application fee.
/// You can do so multiple times, until the entire fee has been refunded.
///
/// Once entirely refunded, an application fee can’t be refunded again.
/// This method will raise an error when called on an already-refunded application fee,
/// or when trying to refund more money than is left on an application fee.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateIdApplicationFeeRefund<'a> {
    inner: CreateIdApplicationFeeRefundBuilder<'a>,
    id: &'a stripe_shared::ApplicationFeeId,
}
impl<'a> CreateIdApplicationFeeRefund<'a> {
    /// Construct a new `CreateIdApplicationFeeRefund`.
    pub fn new(id: &'a stripe_shared::ApplicationFeeId) -> Self {
        Self { id, inner: CreateIdApplicationFeeRefundBuilder::new() }
    }
    /// A positive integer, in _cents (or local equivalent)_, representing how much of this fee to refund.
    /// Can refund only up to the remaining unrefunded amount of the fee.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
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
impl CreateIdApplicationFeeRefund<'_> {
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

impl StripeRequest for CreateIdApplicationFeeRefund<'_> {
    type Output = stripe_shared::ApplicationFeeRefund;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/application_fees/{id}/refunds"))
            .form(&self.inner)
    }
}
