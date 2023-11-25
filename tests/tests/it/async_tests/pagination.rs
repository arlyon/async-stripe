use stripe::Client;
use stripe_connect::account::ListAccount;
use stripe_core::customer::ListCustomer;
use stripe_types::AccountId;

use crate::mock::get_client;

#[tokio::test]
async fn is_account_listable() {
    use futures_util::TryStreamExt;
    let client = get_client();
    let expected_id: AccountId = "acct_1O8RSFF2YyVaukgl".parse().unwrap();

    let result =
        ListAccount::new().paginate().stream(&client).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.first().unwrap().id, expected_id);
}

#[tokio::test]
async fn stream() {
    use futures_util::StreamExt;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    // Start a lightweight mock server.
    let server = MockServer::start_async().await;

    let client = Client::from_url(&*server.url("/"), "fake_key");

    let next_item = server.mock(|when, then| {
        when.method(GET).path("/v1/customers").query_param("starting_after", "cus_1");
        then.status(200).body(
            r#"{"object": "list", "data": [{
                "id": "cus_2",
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
              }], "has_more": false, "url": "/v1/customers"}"#,
        );
    });

    let first_item = server.mock(|when, then| {
        when.method(GET).path("/v1/customers");
        then.status(200).body(
            r#"{"object": "list", "data": [{
                "id": "cus_1",
                "object": "customer",
                "balance": 0,
                "created": 1649316731,
                "currency": "gbp",
                "delinquent": false,
                "invoice_prefix": "4AF7482",
                "invoice_settings": {},
                "livemode": false,
                "metadata": {},
                "preferred_locales": [],
                "tax_exempt": "none"
              }], "has_more": true, "url": "/v1/customers"}"#,
        );
    });

    let paginator = ListCustomer::new().paginate();

    let stream = paginator.stream(&client).collect::<Vec<_>>().await;

    assert_eq!(stream.len(), 2);

    first_item.assert_hits_async(1).await;
    next_item.assert_hits_async(1).await;
}
