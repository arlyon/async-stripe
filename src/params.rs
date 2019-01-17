use crate::config::{err, ok, Client, Response};
use crate::error::Error;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Headers {
    pub stripe_account: Option<String>,
    pub client_id: Option<String>,
}

pub trait Identifiable {
    fn id(&self) -> &str;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            data: self.data.clone(),
            has_more: self.has_more,
            total_count: self.total_count,
            url: self.url.clone(),
        }
    }
}

impl<T: DeserializeOwned + Send + 'static> List<T> {
    /// Prefer `List::next` when possible
    pub fn get_next(client: &Client, url: &str, last_id: &str) -> Response<List<T>> {
        if url.starts_with("/v1/") {
            // TODO: Maybe parse the URL?  Perhaps `List` should always parse its `url` field.
            let mut url = url.trim_left_matches("/v1/").to_string();
            if url.contains('?') {
                url.push_str(&format!("&starting_after={}", last_id));
            } else {
                url.push_str(&format!("?starting_after={}", last_id));
            }
            client.get(&url)
        } else {
            err(Error::Unsupported("URL for fetching additional data uses different API version"))
        }
    }
}

impl<T: Identifiable + DeserializeOwned + Send + 'static> List<T> {
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Not supported by `stripe::async::Client`.
    #[cfg(not(feature = "async"))]
    pub fn get_all(self, client: &Client) -> Response<Vec<T>> {
        let mut data = Vec::new();
        let mut next = self;
        loop {
            if next.has_more {
                let resp = next.next(&client)?;
                data.extend(next.data);
                next = resp;
            } else {
                data.extend(next.data);
                break;
            }
        }
        Ok(data)
    }

    /// Fetch additional page of data from stripe
    pub fn next(&self, client: &Client) -> Response<List<T>> {
        if let Some(last_id) = self.data.last().map(|d| d.id()) {
            List::get_next(client, &self.url, last_id)
        } else {
            ok(List {
                data: Vec::new(),
                has_more: false,
                total_count: self.total_count,
                url: self.url.clone(),
            })
        }
    }
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RangeBounds<T> {
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
}

impl<T> Default for RangeBounds<T> {
    fn default() -> Self {
        RangeBounds { gt: None, gte: None, lt: None, lte: None }
    }
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RangeQuery<T> {
    Exact(T),
    Bounds(RangeBounds<T>),
}

impl<T> RangeQuery<T> {
    /// Filter results to exactly match a given value
    pub fn eq(value: T) -> RangeQuery<T> {
        RangeQuery::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before to a given value
    pub fn lt(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }
}

// NOTE: Only intended to handle conversion from ASCII CamelCase to SnakeCase
//   This function is used to convert static Rust identifiers to snakecase
// TODO: pub(crate) fn
pub fn to_snakecase(camel: &str) -> String {
    let mut i = 0;
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            snake.push(ch);
        }
        i += 1;
    }

    snake
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_snakecase() {
        use super::to_snakecase;

        assert_eq!(to_snakecase("snake_case").as_str(), "snake_case");
        assert_eq!(to_snakecase("CamelCase").as_str(), "camel_case");
        assert_eq!(to_snakecase("XMLHttpRequest").as_str(), "xml_http_request");
        assert_eq!(to_snakecase("UPPER").as_str(), "upper");
        assert_eq!(to_snakecase("lower").as_str(), "lower");
    }
}
