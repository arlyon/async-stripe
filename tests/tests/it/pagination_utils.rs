//! Utilities defined to help test different pagination scenarios more easily.
use std::cmp::min;

use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

/// Basic mock customer data
fn mock_customer_data(customer_id: &str) -> Value {
    json!({
        "id": customer_id,
        "object": "customer",
        "balance": 0,
        "created": 1649316731,
        "currency": "gbp",
        "delinquent": false,
        "email": null,
        "invoice_prefix": "4AF7482",
        "invoice_settings": {},
        "livemode": false,
        "metadata": {},
        "preferred_locales": [],
        "tax_exempt": "none"
    })
}

fn mock_customer_list(customer_ids: &[String], has_more: bool) -> Value {
    let data = customer_ids.iter().map(|id| mock_customer_data(id)).collect::<Vec<_>>();
    json!({
        "object": "list",
        "data": data,
        "has_more": has_more,
        "url": "/v1/customers"
    })
}

fn mock_customer_search_list(
    customer_ids: &[String],
    has_more: bool,
    next_page: Option<&String>,
) -> Value {
    let data = customer_ids.iter().map(|id| mock_customer_data(id)).collect::<Vec<_>>();
    json!({
        "object": "search_result",
        "data": data,
        "has_more": has_more,
        "url": "/v1/customers/search",
        "next_page": next_page
    })
}

fn extract_params(req: &Request) -> serde_json::Map<String, Value> {
    if let Some(query) = req.url.query() {
        serde_qs::from_str(query).expect("invalid list customer request")
    } else {
        Default::default()
    }
}

/// Extract an index from a customer id in the format expected by our mocks
pub fn parse_cus_id(cus_id: &str) -> usize {
    cus_id.trim_start_matches("cus_").parse().expect("expected usize")
}

/// Construct a customer id from an index in the format expected by our mocks
pub fn cons_cus_id(id: usize) -> String {
    format!("cus_{id}")
}

/// Are we testing `List<T>` or `SearchList<T>`?
#[derive(Copy, Clone, Debug)]
pub enum PaginationMockKind {
    List,
    Search,
}

impl PaginationMockKind {
    /// Expected pagination cursor for this kind of operation
    fn cursor_name(self) -> &'static str {
        match self {
            Self::List => "starting_after",
            Self::Search => "page",
        }
    }

    fn extract_cursor_value(self, value: &serde_json::Map<String, Value>) -> Option<String> {
        Some(value.get(self.cursor_name())?.as_str().unwrap().to_string())
    }

    fn api_url(self) -> &'static str {
        match self {
            Self::List => "/v1/customers",
            Self::Search => "/v1/customers/search",
        }
    }
}

/// A super-simple mock server for testing pagination for either list or search requests
///
/// Given a customer_count of `n`, the server will pretend it has mocked customer data
/// with ids `cus_1, cus_2, ..., cus_n`.
///
/// For either the `starting_after` or `page` cursor of `cus_n`, the server will respond
/// with `cus_n+1, cus_n+2, ...` until either the `limit` param is reached or the last item is returned.
///
/// This naive pagination should still ensure cursors are set correctly, and we make the correct number
/// of requests and paginate items in the right order.
#[derive(Debug)]
pub struct PaginationMock {
    server: MockServer,
    customer_count: usize,
    kind: PaginationMockKind,
}

impl PaginationMock {
    pub async fn new(customer_count: usize, kind: PaginationMockKind) -> Self {
        let server = MockServer::start().await;
        let mock = Mock::given(method("GET")).and(path(kind.api_url())).respond_with(
            move |req: &Request| {
                let params = extract_params(req);
                let limit: usize = params
                    .get("limit")
                    .map(|s| s.as_str().unwrap().parse().expect("invalid limit"))
                    .unwrap_or(10);
                let next_customer_id: usize =
                    if let Some(cursor) = kind.extract_cursor_value(&params) {
                        parse_cus_id(&cursor) + 1
                    } else {
                        1
                    };

                let last_customer_id = min(customer_count, next_customer_id + limit - 1);
                let has_more = last_customer_id != customer_count;
                let id_batch_to_return =
                    (next_customer_id..=last_customer_id).map(cons_cus_id).collect::<Vec<_>>();

                if has_more {
                    assert_eq!(id_batch_to_return.len(), limit);
                }
                let mock_ret = match kind {
                    PaginationMockKind::List => mock_customer_list(&id_batch_to_return, has_more),
                    PaginationMockKind::Search => mock_customer_search_list(
                        &id_batch_to_return,
                        has_more,
                        id_batch_to_return.last(),
                    ),
                };
                ResponseTemplate::new(200).set_body_json(mock_ret)
            },
        );
        server.register(mock).await;
        Self { server, customer_count, kind }
    }

    pub fn url(&self) -> String {
        self.server.uri()
    }

    /// All ids we should have collected if we paginated from scratch and got everything
    pub fn all_ids(&self) -> Vec<String> {
        (1..=self.customer_count).map(cons_cus_id).collect()
    }

    /// All ids we should have collected if we paginated everything starting from
    /// `after`
    pub fn all_ids_after(&self, after: usize) -> Vec<String> {
        (after + 1..=self.customer_count).map(cons_cus_id).collect()
    }

    /// Check that we received the expected requests. For example, `expected` might be
    /// [None, 1, 4, 7], signaling that our mock saw 4 requests, first one without
    /// a cursor, then `cus_1`, `cus_4`, `cus_7`.
    pub async fn assert_cursors_received(&self, expected: &[Option<usize>]) {
        let requests = self.server.received_requests().await.expect("we are recording requests");
        for (req, expected) in requests.iter().zip(expected) {
            let params = extract_params(req);
            if let Some(cursor) = self.kind.extract_cursor_value(&params) {
                assert_eq!(Some(parse_cus_id(&cursor)), *expected)
            } else {
                assert!(
                    expected.is_none(),
                    "Received no query param, expected {}",
                    expected.unwrap()
                );
            }
        }
    }
}
