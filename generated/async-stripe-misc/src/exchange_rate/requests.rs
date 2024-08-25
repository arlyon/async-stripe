use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListExchangeRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListExchangeRateBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of objects that contain the rates at which foreign currencies are converted to one another.
/// Only shows the currencies for which Stripe supports.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListExchangeRate {
    inner: ListExchangeRateBuilder,
}
impl ListExchangeRate {
    /// Construct a new `ListExchangeRate`.
    pub fn new() -> Self {
        Self { inner: ListExchangeRateBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with the exchange rate for currency X your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
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
    /// Limit can range between 1 and total number of supported payout currencies, and the default is the max.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with the exchange rate for currency X, your subsequent call can include `starting_after=X` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListExchangeRate {
    fn default() -> Self {
        Self::new()
    }
}
impl ListExchangeRate {
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
        stripe_client_core::ListPaginator::new_list("/exchange_rates", &self.inner)
    }
}

impl StripeRequest for ListExchangeRate {
    type Output = stripe_types::List<stripe_misc::ExchangeRate>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/exchange_rates").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveExchangeRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveExchangeRateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the exchange rates from the given currency to every supported currency.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveExchangeRate {
    inner: RetrieveExchangeRateBuilder,
    rate_id: stripe_misc::ExchangeRateId,
}
impl RetrieveExchangeRate {
    /// Construct a new `RetrieveExchangeRate`.
    pub fn new(rate_id: impl Into<stripe_misc::ExchangeRateId>) -> Self {
        Self { rate_id: rate_id.into(), inner: RetrieveExchangeRateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveExchangeRate {
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

impl StripeRequest for RetrieveExchangeRate {
    type Output = stripe_misc::ExchangeRate;

    fn build(&self) -> RequestBuilder {
        let rate_id = &self.rate_id;
        RequestBuilder::new(StripeMethod::Get, format!("/exchange_rates/{rate_id}"))
            .query(&self.inner)
    }
}
