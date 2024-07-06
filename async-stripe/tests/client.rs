use httpmock::Method::{GET, POST};
use httpmock::MockServer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stripe::{Client, ClientBuilder, RequestStrategy, StripeError};
use stripe_client_core::{CustomizableStripeRequest, RequestBuilder, StripeMethod};
use stripe_shared::ApiErrorsType::InvalidRequestError;
use stripe_shared::{AccountId, ApiErrorsCode, ApplicationId};

fn client_builder() -> ClientBuilder {
    ClientBuilder::new("secret")
}

fn get_client_for(mock_server: &MockServer) -> Client {
    client_builder().url(mock_server.base_url()).build().expect("could not build client")
}

fn server_errors_req() -> CustomizableStripeRequest<()> {
    RequestBuilder::new(StripeMethod::Get, "/server-errors").customize()
}

fn test_req() -> CustomizableStripeRequest<TestData> {
    RequestBuilder::new(StripeMethod::Get, "/test").customize()
}

#[derive(miniserde::Deserialize, Debug, Serialize, Deserialize)]
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

#[tokio::test]
async fn retry() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = client_builder()
        .url(server.base_url())
        .request_strategy(RequestStrategy::Retry(5))
        .build()
        .unwrap();

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500);
    });

    let res = server_errors_req().send(&client).await;

    mock.assert_hits_async(5).await;
    assert!(res.is_err());

    // And it should also work for retry strategy set per request
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500);
    });

    let res = server_errors_req().request_strategy(RequestStrategy::Retry(5)).send(&client).await;

    mock.assert_hits_async(5).await;
    assert!(res.is_err());
}

#[tokio::test]
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

    let res = RequestBuilder::new(StripeMethod::Get, "/missing")
        .customize::<TestData>()
        .request_strategy(RequestStrategy::Retry(3))
        .send(&client)
        .await;

    // We don't retry a 404
    mock.assert_hits_async(1).await;

    match res {
        Err(StripeError::Stripe(x, status)) => {
            assert_eq!(status, 404);
            assert_eq!(x.type_, InvalidRequestError);
            assert!(x.message.unwrap().contains("Unrecognized"));
        }
        _ => panic!("Expected stripe error"),
    }
}

#[tokio::test]
async fn nice_serde_error() {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, miniserde::Deserialize)]
    struct DataType {
        // Allowing dead code since used for deserialization
        #[allow(dead_code)]
        id: String,
        #[allow(dead_code)]
        name: String,
    }

    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/odd_data");
        then.status(200).body(
            "{
                \"id\": \"test\",
                \"name\": 10
              }
              ",
        );
    });

    let req = RequestBuilder::new(StripeMethod::Get, "/odd_data")
        .customize::<DataType>()
        .request_strategy(RequestStrategy::Retry(3));
    let res = req.send(&client).await;

    mock.assert_hits_async(1).await;

    match res {
        Err(StripeError::JSONDeserialize(_)) => {}
        _ => panic!("Expected stripe error {:?}", res),
    }
}

#[tokio::test]
async fn retry_header() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500).header("Stripe-Should-Retry", "false");
    });

    let res = server_errors_req().send(&client).await;

    hello_mock.assert_hits_async(1).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn retry_with_body() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/server-errors")
            .header("content-type", "application/x-www-form-urlencoded")
            .x_www_form_urlencoded_tuple("id", TEST_DATA_ID);
        then.status(500);
    });

    let req = RequestBuilder::new(StripeMethod::Post, "/server-errors").form(&TestData::new());
    let res =
        req.customize::<TestData>().request_strategy(RequestStrategy::Retry(5)).send(&client).await;

    mock.assert_hits_async(5).await;
    assert!(res.is_err());
}

// https://github.com/arlyon/async-stripe/issues/384
#[tokio::test]
async fn user_error_transfers() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    let message = "Your destination account needs to have at least one of the following capabilities enabled: transfers, crypto_transfers, legacy_payments";
    let log_url = "https://dashboard.stripe.com/logs/req_nIhlutaV4amLEs?t=1685040634";
    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/transfers");
        then.status(400).json_body(json!({
          "error": {
            "code": "insufficient_capabilities_for_transfer",
            "message": message,
            "request_log_url": log_url,
            "type": "invalid_request_error"
          }
        }));
    });

    let req = RequestBuilder::new(StripeMethod::Get, "/transfers").customize::<()>();
    let res = req.send(&client).await.unwrap_err();
    mock.assert_hits_async(1).await;

    match res {
        StripeError::Stripe(err, status_code) => {
            assert_eq!(status_code, 400);
            assert_eq!(err.type_, InvalidRequestError);
            assert_eq!(err.message.as_deref(), Some(message));
            assert_eq!(err.request_log_url.as_deref(), Some(log_url));
            // NB: `Unknown` here because the error code reported in the issue is not
            // present in the OpenAPI spec. Reporting unknown instead of an error seems
            // better regardless so that stripe adding new variants is not a breaking change
            assert_eq!(err.code, Some(ApiErrorsCode::Unknown));
        }
        _ => panic!("Expected stripe error, got {:?}", res),
    }
}

async fn assert_headers_sent(
    builder: ClientBuilder,
    req: CustomizableStripeRequest<TestData>,
    expected: Vec<(impl Into<String>, impl Into<String>)>,
) {
    let server = MockServer::start_async().await;
    let client = builder.url(server.base_url()).build().unwrap();

    let mock = server.mock(|mut when, then| {
        when = when.method(GET).path("/v1/test");
        for (name, value) in expected {
            when = when.header(name, value);
        }

        then.status(200).json_body_obj(&TestData::new());
    });

    let _ = req.send(&client).await;
    mock.assert_hits_async(1).await;
}

async fn assert_user_agent_sent(builder: ClientBuilder, expected: String) {
    assert_headers_sent(builder, test_req(), vec![("user-agent", expected)]).await
}

/// Ensures our user agent matches the format of the other stripe clients.
///
/// See: <https://github.com/stripe/stripe-python/blob/3b917dc4cec6a3cccfd46961e05fe7b55c6bee87/stripe/api_requestor.py#L241>
#[tokio::test]
async fn user_agent() {
    assert_user_agent_sent(
        client_builder(),
        format!("Stripe/v1 RustBindings/{}", env!("CARGO_PKG_VERSION")),
    )
    .await;
    assert_user_agent_sent(
        client_builder().app_info("sick-new-startup", None, None),
        format!("Stripe/v1 RustBindings/{} sick-new-startup", env!("CARGO_PKG_VERSION")),
    )
    .await;
    assert_user_agent_sent(
        client_builder().app_info(
            "sick-new-startup",
            Some("0.1.0".to_string()),
            Some("https://sick-startup.io".to_string()),
        ),
        format!(
            "Stripe/v1 RustBindings/{} sick-new-startup/0.1.0 (https://sick-startup.io)",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .await;
}

async fn assert_header_ids(
    builder: ClientBuilder,
    req: CustomizableStripeRequest<TestData>,
    expected_client_id: Option<&ApplicationId>,
    expected_account_id: Option<&AccountId>,
) {
    let mut headers = vec![];
    if let Some(client_id) = expected_client_id {
        headers.push(("client-id", client_id.as_str()));
    }
    if let Some(acct_id) = expected_account_id {
        headers.push(("stripe-account", acct_id.as_str()));
    }
    assert_headers_sent(builder, req, headers).await;
}

#[tokio::test]
async fn client_and_account_ids() {
    let client_id = ApplicationId::from("app_id".to_string());
    let account_id = AccountId::from("acct_id".to_string());
    let both = client_builder().client_id(client_id.clone()).account_id(account_id.clone());
    assert_header_ids(both.clone(), test_req(), Some(&client_id), Some(&account_id)).await;

    let acct_id_builder = client_builder().account_id(account_id.clone());
    assert_header_ids(acct_id_builder, test_req(), None, Some(&account_id)).await;

    // We should also be able to override the account id on a per-request basis
    let new_account_id = AccountId::from("acct_overridden".to_string());
    assert_header_ids(
        both,
        test_req().account_id(new_account_id.clone()),
        Some(&client_id),
        Some(&new_account_id),
    )
    .await;
}
