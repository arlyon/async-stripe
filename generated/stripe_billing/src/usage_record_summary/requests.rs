#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListUsageRecordSummary<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListUsageRecordSummary<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> stripe::PaginationParams for ListUsageRecordSummary<'a> {}
impl<'a> ListUsageRecordSummary<'a> {
    /// For the specified subscription item, returns a list of summary objects.
    ///
    /// Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).  The list is sorted in reverse-chronological order (newest first).
    /// The first list item represents the most current usage period that hasn’t ended yet.
    /// Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
    pub fn send(
        &self,
        client: &stripe::Client,
        subscription_item: &stripe_types::subscription_item::SubscriptionItemId,
    ) -> stripe::Response<stripe_types::List<stripe_types::UsageRecordSummary>> {
        client.get_query(
            &format!(
                "/subscription_items/{subscription_item}/usage_record_summaries",
                subscription_item = subscription_item
            ),
            self,
        )
    }
    pub fn paginate(
        self,
        subscription_item: &stripe_types::subscription_item::SubscriptionItemId,
    ) -> stripe::ListPaginator<stripe_types::UsageRecordSummary> {
        stripe::ListPaginator::from_params(
            &format!(
                "/subscription_items/{subscription_item}/usage_record_summaries",
                subscription_item = subscription_item
            ),
            self,
        )
    }
}
