use serde::Serialize;

use crate::params::SearchList;
use crate::{Client, Price, Response};

#[derive(Clone, Debug, Default, Serialize)]
pub struct PriceSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> PriceSearchParams<'a> {
    pub fn new() -> PriceSearchParams<'a> {
        PriceSearchParams { query: String::new(), limit: None, page: None, expand: &[] }
    }
}

impl Price {
    /// Searches for a price.
    ///
    /// For more details see <https://stripe.com/docs/api/prices/search>.
    pub fn search(client: &Client, params: PriceSearchParams) -> Response<SearchList<Price>> {
        client.get_query("/prices/search", params)
    }
}
