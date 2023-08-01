use serde::Serialize;

use crate::params::SearchList;
use crate::{Client, Product, Response};

#[derive(Clone, Debug, Default, Serialize)]
pub struct ProductSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> ProductSearchParams<'a> {
    pub fn new() -> ProductSearchParams<'a> {
        ProductSearchParams { query: String::new(), limit: None, page: None, expand: &[] }
    }
}

impl Product {
    /// Searches for a product.
    ///
    /// For more details see <https://stripe.com/docs/api/products/search>.
    pub fn search(client: &Client, params: ProductSearchParams) -> Response<SearchList<Product>> {
        client.get_query("/products/search", params)
    }
}
