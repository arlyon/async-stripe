use stripe::{Client, Response};

impl stripe_misc::exchange_rate::ExchangeRate {
    /// Returns a list of objects that contain the rates at which foreign currencies are converted to one another.
    ///
    /// Only shows the currencies for which Stripe supports.
    pub fn list(
        client: &Client,
        params: ListExchangeRate,
    ) -> Response<stripe_types::List<stripe_misc::exchange_rate::ExchangeRate>> {
        client.get_query("/exchange_rates", params)
    }
    /// Retrieves the exchange rates from the given currency to every supported currency.
    pub fn retrieve(
        client: &Client,
        rate_id: &str,
        params: RetrieveExchangeRate,
    ) -> Response<stripe_misc::exchange_rate::ExchangeRate> {
        client.get_query(&format!("/exchange_rates/{rate_id}", rate_id = rate_id), params)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListExchangeRate<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with the exchange rate for currency X your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and total number of supported payout currencies, and the default is the max.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with the exchange rate for currency X, your subsequent call can include `starting_after=X` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListExchangeRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveExchangeRate<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveExchangeRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}