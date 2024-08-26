use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListSubscriptionItemUsageRecordSummaryBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListSubscriptionItemUsageRecordSummaryBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// For the specified subscription item, returns a list of summary objects.
/// Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).
///
/// The list is sorted in reverse-chronological order (newest first).
/// The first list item represents the most current usage period that hasn’t ended yet.
/// Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListSubscriptionItemUsageRecordSummary {
    inner: ListSubscriptionItemUsageRecordSummaryBuilder,
    subscription_item: stripe_shared::SubscriptionItemId,
}
impl ListSubscriptionItemUsageRecordSummary {
    /// Construct a new `ListSubscriptionItemUsageRecordSummary`.
    pub fn new(subscription_item: impl Into<stripe_shared::SubscriptionItemId>) -> Self {
        Self {
            subscription_item: subscription_item.into(),
            inner: ListSubscriptionItemUsageRecordSummaryBuilder::new(),
        }
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
impl ListSubscriptionItemUsageRecordSummary {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::UsageRecordSummary>>
    {
        let subscription_item = &self.subscription_item;

        stripe_client_core::ListPaginator::new_list(
            format!("/subscription_items/{subscription_item}/usage_record_summaries"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListSubscriptionItemUsageRecordSummary {
    type Output = stripe_types::List<stripe_shared::UsageRecordSummary>;

    fn build(&self) -> RequestBuilder {
        let subscription_item = &self.subscription_item;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/subscription_items/{subscription_item}/usage_record_summaries"),
        )
        .query(&self.inner)
    }
}
