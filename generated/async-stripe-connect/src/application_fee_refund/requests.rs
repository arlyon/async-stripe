use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveApplicationFeeRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveApplicationFeeRefundBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// By default, you can see the 10 most recent refunds stored directly on the application fee object, but you can also retrieve details about a specific refund stored on the application fee.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveApplicationFeeRefund {
    inner: RetrieveApplicationFeeRefundBuilder,
    fee: stripe_shared::ApplicationFeeId,
    id: String,
}
impl RetrieveApplicationFeeRefund {
    /// Construct a new `RetrieveApplicationFeeRefund`.
    pub fn new(fee: impl Into<stripe_shared::ApplicationFeeId>, id: impl Into<String>) -> Self {
        Self { fee: fee.into(), id: id.into(), inner: RetrieveApplicationFeeRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveApplicationFeeRefund {
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

impl StripeRequest for RetrieveApplicationFeeRefund {
    type Output = stripe_shared::ApplicationFeeRefund;

    fn build(&self) -> RequestBuilder {
        let fee = &self.fee;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/application_fees/{fee}/refunds/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListIdApplicationFeeRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListIdApplicationFeeRefundBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// You can see a list of the refunds belonging to a specific application fee.
/// Note that the 10 most recent refunds are always available by default on the application fee object.
/// If you need more than those 10, you can use this API method and the `limit` and `starting_after` parameters to page through additional refunds.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIdApplicationFeeRefund {
    inner: ListIdApplicationFeeRefundBuilder,
    id: stripe_shared::ApplicationFeeId,
}
impl ListIdApplicationFeeRefund {
    /// Construct a new `ListIdApplicationFeeRefund`.
    pub fn new(id: impl Into<stripe_shared::ApplicationFeeId>) -> Self {
        Self { id: id.into(), inner: ListIdApplicationFeeRefundBuilder::new() }
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
impl ListIdApplicationFeeRefund {
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
        let id = &self.id;

        stripe_client_core::ListPaginator::new_list(
            format!("/application_fees/{id}/refunds"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListIdApplicationFeeRefund {
    type Output = stripe_types::List<stripe_shared::ApplicationFeeRefund>;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/application_fees/{id}/refunds"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateApplicationFeeRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateApplicationFeeRefundBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified application fee refund by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
///
/// This request only accepts metadata as an argument.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateApplicationFeeRefund {
    inner: UpdateApplicationFeeRefundBuilder,
    fee: stripe_shared::ApplicationFeeId,
    id: String,
}
impl UpdateApplicationFeeRefund {
    /// Construct a new `UpdateApplicationFeeRefund`.
    pub fn new(fee: impl Into<stripe_shared::ApplicationFeeId>, id: impl Into<String>) -> Self {
        Self { fee: fee.into(), id: id.into(), inner: UpdateApplicationFeeRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
}
impl UpdateApplicationFeeRefund {
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

impl StripeRequest for UpdateApplicationFeeRefund {
    type Output = stripe_shared::ApplicationFeeRefund;

    fn build(&self) -> RequestBuilder {
        let fee = &self.fee;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/application_fees/{fee}/refunds/{id}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateIdApplicationFeeRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl CreateIdApplicationFeeRefundBuilder {
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
pub struct CreateIdApplicationFeeRefund {
    inner: CreateIdApplicationFeeRefundBuilder,
    id: stripe_shared::ApplicationFeeId,
}
impl CreateIdApplicationFeeRefund {
    /// Construct a new `CreateIdApplicationFeeRefund`.
    pub fn new(id: impl Into<stripe_shared::ApplicationFeeId>) -> Self {
        Self { id: id.into(), inner: CreateIdApplicationFeeRefundBuilder::new() }
    }
    /// A positive integer, in _cents (or local equivalent)_, representing how much of this fee to refund.
    /// Can refund only up to the remaining unrefunded amount of the fee.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
}
impl CreateIdApplicationFeeRefund {
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

impl StripeRequest for CreateIdApplicationFeeRefund {
    type Output = stripe_shared::ApplicationFeeRefund;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/application_fees/{id}/refunds"))
            .form(&self.inner)
    }
}
