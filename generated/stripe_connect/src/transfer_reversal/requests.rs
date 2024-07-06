use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIdTransferReversalBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListIdTransferReversalBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// You can see a list of the reversals belonging to a specific transfer.
/// Note that the 10 most recent reversals are always available by default on the transfer object.
/// If you need more than those 10, you can use this API method and the `limit` and `starting_after` parameters to page through additional reversals.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIdTransferReversal<'a> {
    inner: ListIdTransferReversalBuilder<'a>,
    id: &'a stripe_shared::TransferId,
}
impl<'a> ListIdTransferReversal<'a> {
    /// Construct a new `ListIdTransferReversal`.
    pub fn new(id: &'a stripe_shared::TransferId) -> Self {
        Self { id, inner: ListIdTransferReversalBuilder::new() }
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
impl ListIdTransferReversal<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::TransferReversal>>
    {
        let id = self.id;

        stripe_client_core::ListPaginator::new_list(
            format!("/transfers/{id}/reversals"),
            self.inner,
        )
    }
}

impl StripeRequest for ListIdTransferReversal<'_> {
    type Output = stripe_types::List<stripe_shared::TransferReversal>;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/transfers/{id}/reversals"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTransferReversalBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTransferReversalBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// By default, you can see the 10 most recent reversals stored directly on the transfer object, but you can also retrieve details about a specific reversal stored on the transfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTransferReversal<'a> {
    inner: RetrieveTransferReversalBuilder<'a>,
    id: &'a str,
    transfer: &'a stripe_shared::TransferId,
}
impl<'a> RetrieveTransferReversal<'a> {
    /// Construct a new `RetrieveTransferReversal`.
    pub fn new(id: &'a str, transfer: &'a stripe_shared::TransferId) -> Self {
        Self { id, transfer, inner: RetrieveTransferReversalBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTransferReversal<'_> {
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

impl StripeRequest for RetrieveTransferReversal<'_> {
    type Output = stripe_shared::TransferReversal;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        let transfer = self.transfer;
        RequestBuilder::new(StripeMethod::Get, format!("/transfers/{transfer}/reversals/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateIdTransferReversalBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_application_fee: Option<bool>,
}
impl<'a> CreateIdTransferReversalBuilder<'a> {
    fn new() -> Self {
        Self {
            amount: None,
            description: None,
            expand: None,
            metadata: None,
            refund_application_fee: None,
        }
    }
}
/// When you create a new reversal, you must specify a transfer to create it on.
///
/// When reversing transfers, you can optionally reverse part of the transfer.
/// You can do so as many times as you wish until the entire transfer has been reversed.
///
/// Once entirely reversed, a transfer can’t be reversed again.
/// This method will return an error when called on an already-reversed transfer, or when trying to reverse more money than is left on a transfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateIdTransferReversal<'a> {
    inner: CreateIdTransferReversalBuilder<'a>,
    id: &'a stripe_shared::TransferId,
}
impl<'a> CreateIdTransferReversal<'a> {
    /// Construct a new `CreateIdTransferReversal`.
    pub fn new(id: &'a stripe_shared::TransferId) -> Self {
        Self { id, inner: CreateIdTransferReversalBuilder::new() }
    }
    /// A positive integer in cents (or local equivalent) representing how much of this transfer to reverse.
    /// Can only reverse up to the unreversed amount remaining of the transfer.
    /// Partial transfer reversals are only allowed for transfers to Stripe Accounts.
    /// Defaults to the entire transfer amount.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// An arbitrary string which you can attach to a reversal object.
    /// It is displayed alongside the reversal in the Dashboard.
    /// This will be unset if you POST an empty value.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
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
    /// Boolean indicating whether the application fee should be refunded when reversing this transfer.
    /// If a full transfer reversal is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded with an amount proportional to the amount of the transfer reversed.
    pub fn refund_application_fee(mut self, refund_application_fee: bool) -> Self {
        self.inner.refund_application_fee = Some(refund_application_fee);
        self
    }
}
impl CreateIdTransferReversal<'_> {
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

impl StripeRequest for CreateIdTransferReversal<'_> {
    type Output = stripe_shared::TransferReversal;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/transfers/{id}/reversals"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTransferReversalBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTransferReversalBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified reversal by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
///
/// This request only accepts metadata and description as arguments.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTransferReversal<'a> {
    inner: UpdateTransferReversalBuilder<'a>,
    id: &'a str,
    transfer: &'a stripe_shared::TransferId,
}
impl<'a> UpdateTransferReversal<'a> {
    /// Construct a new `UpdateTransferReversal`.
    pub fn new(id: &'a str, transfer: &'a stripe_shared::TransferId) -> Self {
        Self { id, transfer, inner: UpdateTransferReversalBuilder::new() }
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
impl UpdateTransferReversal<'_> {
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

impl StripeRequest for UpdateTransferReversal<'_> {
    type Output = stripe_shared::TransferReversal;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        let transfer = self.transfer;
        RequestBuilder::new(StripeMethod::Post, format!("/transfers/{transfer}/reversals/{id}"))
            .form(&self.inner)
    }
}
