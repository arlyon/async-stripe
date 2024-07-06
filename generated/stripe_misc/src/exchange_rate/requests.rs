use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListExchangeRateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListExchangeRateBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of objects that contain the rates at which foreign currencies are converted to one another.
/// Only shows the currencies for which Stripe supports.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListExchangeRate<'a> {
    inner: ListExchangeRateBuilder<'a>,
}
impl<'a> ListExchangeRate<'a> {
    /// Construct a new `ListExchangeRate`.
    pub fn new() -> Self {
        Self { inner: ListExchangeRateBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with the exchange rate for currency X your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
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
    /// Limit can range between 1 and total number of supported payout currencies, and the default is the max.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with the exchange rate for currency X, your subsequent call can include `starting_after=X` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListExchangeRate<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListExchangeRate<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::ExchangeRate>> {
        stripe_client_core::ListPaginator::new_list("/exchange_rates", self.inner)
    }
}

impl StripeRequest for ListExchangeRate<'_> {
    type Output = stripe_types::List<stripe_misc::ExchangeRate>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/exchange_rates").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveExchangeRateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveExchangeRateBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the exchange rates from the given currency to every supported currency.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveExchangeRate<'a> {
    inner: RetrieveExchangeRateBuilder<'a>,
    rate_id: &'a stripe_misc::ExchangeRateId,
}
impl<'a> RetrieveExchangeRate<'a> {
    /// Construct a new `RetrieveExchangeRate`.
    pub fn new(rate_id: &'a stripe_misc::ExchangeRateId) -> Self {
        Self { rate_id, inner: RetrieveExchangeRateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveExchangeRate<'_> {
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

impl StripeRequest for RetrieveExchangeRate<'_> {
    type Output = stripe_misc::ExchangeRate;

    fn build(&self) -> RequestBuilder {
        let rate_id = self.rate_id;
        RequestBuilder::new(StripeMethod::Get, format!("/exchange_rates/{rate_id}"))
            .query(&self.inner)
    }
}
