#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListPaymentFlowsSetupIntentSetupAttempt<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp or a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// Only return SetupAttempts created by the SetupIntent specified by
    /// this ID.
    pub setup_intent: &'a str,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListPaymentFlowsSetupIntentSetupAttempt<'a> {
    pub fn new(setup_intent: &'a str) -> Self {
        Self {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            setup_intent,
            starting_after: Default::default(),
        }
    }
}
impl<'a> ListPaymentFlowsSetupIntentSetupAttempt<'a> {
    /// Returns a list of SetupAttempts that associate with a provided SetupIntent.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::PaymentFlowsSetupIntentSetupAttempt>>
    {
        client.get_query("/setup_attempts", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::PaymentFlowsSetupIntentSetupAttempt> {
        stripe::ListPaginator::from_params("/setup_attempts", self)
    }
}
impl<'a> stripe::PaginationParams for ListPaymentFlowsSetupIntentSetupAttempt<'a> {}
