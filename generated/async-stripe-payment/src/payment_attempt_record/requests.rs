use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListPaymentAttemptRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    payment_record: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListPaymentAttemptRecordBuilder {
    fn new(payment_record: impl Into<String>) -> Self {
        Self {
            expand: None,
            limit: None,
            payment_record: payment_record.into(),
            starting_after: None,
        }
    }
}
/// List all the Payment Attempt Records attached to the specified Payment Record.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentAttemptRecord {
    inner: ListPaymentAttemptRecordBuilder,
}
impl ListPaymentAttemptRecord {
    /// Construct a new `ListPaymentAttemptRecord`.
    pub fn new(payment_record: impl Into<String>) -> Self {
        Self { inner: ListPaymentAttemptRecordBuilder::new(payment_record.into()) }
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
impl ListPaymentAttemptRecord {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_payment::PaymentAttemptRecord>>
    {
        stripe_client_core::ListPaginator::new_list("/payment_attempt_records", &self.inner)
    }
}

impl StripeRequest for ListPaymentAttemptRecord {
    type Output = stripe_types::List<stripe_payment::PaymentAttemptRecord>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_attempt_records").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrievePaymentAttemptRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePaymentAttemptRecordBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Payment Attempt Record with the given ID
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentAttemptRecord {
    inner: RetrievePaymentAttemptRecordBuilder,
    id: stripe_payment::PaymentAttemptRecordId,
}
impl RetrievePaymentAttemptRecord {
    /// Construct a new `RetrievePaymentAttemptRecord`.
    pub fn new(id: impl Into<stripe_payment::PaymentAttemptRecordId>) -> Self {
        Self { id: id.into(), inner: RetrievePaymentAttemptRecordBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePaymentAttemptRecord {
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

impl StripeRequest for RetrievePaymentAttemptRecord {
    type Output = stripe_payment::PaymentAttemptRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_attempt_records/{id}"))
            .query(&self.inner)
    }
}
