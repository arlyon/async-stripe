// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ExchangeRateId};
use crate::params::{Expand, List, Object, Paginable};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ExchangeRate".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeRate {
    /// Unique identifier for the object.
    ///
    /// Represented as the three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) in lowercase.
    pub id: ExchangeRateId,

    /// Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: f64,
}

impl ExchangeRate {

    /// Returns a list of objects that contain the rates at which foreign currencies are converted to one another.
    ///
    /// Only shows the currencies for which Stripe supports.
pub fn list(client: &Client, params: &ListExchangeRates<'_>) -> Response<List<ExchangeRate>> {
   client.get_query("/exchange_rates", params)
}


    /// Retrieves the exchange rates from the given currency to every supported currency.
    pub fn retrieve(client: &Client, id: &ExchangeRateId, expand: &[&str]) -> Response<ExchangeRate> {
        client.get_query(&format!("/exchange_rates/{}", id), Expand { expand })
    }
}

impl Object for ExchangeRate {
    type Id = ExchangeRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "exchange_rate"
    }
}

/// The parameters for `ExchangeRate::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListExchangeRates<'a> {

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with the exchange rate for currency X your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<ExchangeRateId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and total number of supported payout currencies, and the default is the max.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is the currency that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with the exchange rate for currency X, your subsequent call can include `starting_after=X` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<ExchangeRateId>,
}

impl<'a> ListExchangeRates<'a> {
    pub fn new() -> Self {
        ListExchangeRates {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListExchangeRates<'_> {
    type O = ExchangeRate;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}