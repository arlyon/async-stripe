use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListSetupAttemptBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    setup_intent: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListSetupAttemptBuilder<'a> {
    fn new(setup_intent: &'a str) -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            setup_intent,
            starting_after: None,
        }
    }
}
/// Returns a list of SetupAttempts that associate with a provided SetupIntent.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListSetupAttempt<'a> {
    inner: ListSetupAttemptBuilder<'a>,
}
impl<'a> ListSetupAttempt<'a> {
    /// Construct a new `ListSetupAttempt`.
    pub fn new(setup_intent: &'a str) -> Self {
        Self { inner: ListSetupAttemptBuilder::new(setup_intent) }
    }
    /// A filter on the list, based on the object `created` field. The value
    /// can be a string with an integer Unix timestamp or a
    /// dictionary with a number of different query options.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
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
impl ListSetupAttempt<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::SetupAttempt>> {
        stripe_client_core::ListPaginator::new_list("/setup_attempts", self.inner)
    }
}

impl StripeRequest for ListSetupAttempt<'_> {
    type Output = stripe_types::List<stripe_shared::SetupAttempt>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/setup_attempts").query(&self.inner)
    }
}
