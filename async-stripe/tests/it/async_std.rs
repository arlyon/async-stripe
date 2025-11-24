use http_types::convert::{Deserialize, Serialize};
use httpmock::prelude::*;
use stripe::StripeError;
use stripe::async_std::{Client, ClientBuilder};
use stripe_client_core::{
    CustomizableStripeRequest, RequestBuilder, RequestStrategy, StripeMethod,
};
use stripe_shared::ApiErrorsType::InvalidRequestError;

fn client_builder() -> ClientBuilder {
    ClientBuilder::new("secret")
}

fn get_client_for(mock_server: &MockServer) -> Client {
    client_builder().url(mock_server.base_url()).build().expect("could not build client")
}

fn server_errors_req() -> CustomizableStripeRequest<()> {
    RequestBuilder::new(StripeMethod::Get, "/server-errors").customize()
}

#[derive(Debug, Serialize, Deserialize)]
struct TestData {
    // Allowing dead code since used for deserialization
    #[allow(dead_code)]
    id: String,
}

const TEST_DATA_ID: &str = "test-id";

impl TestData {
    pub fn new() -> Self {
        Self { id: TEST_DATA_ID.into() }
    }
}

#[async_std::test]
async fn retry() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500);
    });

    let req = server_errors_req().request_strategy(RequestStrategy::Retry(5));
    let res = req.send(&client).await;

    hello_mock.assert_hits_async(5).await;
    assert!(res.is_err());
}

#[async_std::test]
async fn user_error() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/missing");
        then.status(404).body("{
                \"error\": {
                  \"message\": \"Unrecognized request URL (GET: /v1/missing). Please see https://stripe.com/docs or we can help at https://support.stripe.com/.\",
                  \"type\": \"invalid_request_error\"
                }
              }
              ");
    });

    let req = RequestBuilder::new(StripeMethod::Get, "/missing")
        .customize::<()>()
        .request_strategy(RequestStrategy::Retry(3));
    let res = req.send(&client).await;

    mock.assert_hits_async(1).await;

    match res {
        Err(StripeError::Stripe(x, status)) => {
            assert_eq!(status, 404);
            assert_eq!(x.type_, InvalidRequestError);
            assert!(x.message.unwrap().contains("Unrecognized"));
        }
        _ => panic!("Expected stripe error, got {res:?}"),
    }
}

#[async_std::test]
async fn retry_header() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500).header("Stripe-Should-Retry", "false");
    });

    let req = server_errors_req().request_strategy(RequestStrategy::Retry(3));
    let res = req.send(&client).await;

    hello_mock.assert_hits_async(1).await;
    assert!(res.is_err());
}

#[async_std::test]
async fn retry_body() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/server-errors")
            .header("content-type", "application/x-www-form-urlencoded")
            .x_www_form_urlencoded_tuple("id", TEST_DATA_ID);
        then.status(500);
    });

    let req = RequestBuilder::new(StripeMethod::Post, "/server-errors")
        .form(&TestData::new())
        .customize::<()>()
        .request_strategy(RequestStrategy::Retry(5));
    let res = req.send(&client).await;

    hello_mock.assert_hits_async(5).await;
    assert!(res.is_err());
}
