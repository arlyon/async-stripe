use miniserde::json::from_str;
use serde_json::{Value, json};
use stripe::ClientBuilder;
use stripe_core::Customer;
use stripe_core::customer::{ListCustomer, RetrieveCustomerReturned};
use stripe_types::{List, SearchList};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn customer(id: &str) -> Value {
    json!({"id": id, "object": "customer", "created": 1750000000, "livemode": false})
}

fn page(data: Vec<Value>) -> Value {
    json!({"object": "list", "url": "/v1/customers", "has_more": false, "data": data})
}

#[tokio::test]
async fn strict_deserialization_customer_list_request_rejects_missing_fields() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/customers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(page(vec![
            customer("cus_good01"),
            json!({"id": "cus_bad01", "object": "customer"}),
        ])))
        .expect(1)
        .mount(&server)
        .await;
    let client = ClientBuilder::new("sk_test_placeholder").url(server.uri()).build().unwrap();

    let result = ListCustomer::new().send(&client).await;
    assert!(result.is_err(), "an invalid customer must not silently disappear: {result:?}");
    server.verify().await;
}

#[test]
fn strict_deserialization_rejects_invalid_items_at_every_position() {
    for index in 0..3 {
        let mut data = vec![customer("cus_first"), customer("cus_middle"), customer("cus_last")];
        data[index] = json!({"id": "cus_bad", "object": "customer"});
        let body = page(data).to_string();
        assert!(from_str::<List<Customer>>(&body).is_err(), "list item {index}");
        assert!(from_str::<SearchList<Customer>>(&body).is_err(), "search item {index}");
    }
}

#[test]
fn strict_deserialization_preserves_valid_and_empty_pages() {
    for data in [vec![], vec![customer("cus_first"), customer("cus_second")]] {
        let expected_len = data.len();
        let body = page(data).to_string();
        let list: List<Customer> = from_str(&body).unwrap();
        let search: SearchList<Customer> = from_str(&body).unwrap();
        assert_eq!(list.data.len(), expected_len);
        assert_eq!(search.data.len(), expected_len);
        if expected_len != 0 {
            assert_eq!(list.data[0].id.as_str(), "cus_first");
            assert_eq!(list.data[1].id.as_str(), "cus_second");
            assert_eq!(search.data[0].id.as_str(), "cus_first");
            assert_eq!(search.data[1].id.as_str(), "cus_second");
        }
    }
}

#[test]
fn strict_deserialization_rejects_invalid_object_union_items() {
    let body = json!([
        customer("cus_good"),
        {"id": "cus_bad", "object": "customer"}
    ])
    .to_string();
    assert!(from_str::<Vec<RetrieveCustomerReturned>>(&body).is_err());
}

#[test]
fn strict_deserialization_preserves_valid_object_union_items() {
    let body = json!([
        customer("cus_good"),
        {"id": "cus_deleted", "object": "customer", "deleted": true}
    ])
    .to_string();
    assert_eq!(from_str::<Vec<RetrieveCustomerReturned>>(&body).unwrap().len(), 2);
}

#[test]
fn strict_deserialization_rejects_invalid_nested_vector_items() {
    let mut value = customer("cus_parent");
    value["sources"] = json!({
        "object": "list",
        "url": "/v1/customers/cus_parent/sources",
        "has_more": false,
        "data": [{"object": "card", "id": "card_bad"}]
    });
    assert!(from_str::<Customer>(&value.to_string()).is_err());
}

#[test]
fn strict_deserialization_rejects_invalid_standalone_customer() {
    assert!(from_str::<Customer>(r#"{"id":"cus_bad","object":"customer"}"#).is_err());
}
