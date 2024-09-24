#![recursion_limit = "256"]
use serde_json::json;

#[test]
fn debug_currency() {
    use stripe::Currency;
    assert_eq!(format!("{:?}", Currency::AED), "AED");
    assert_eq!(format!("{:?}", Currency::USD), "USD");
    assert_eq!(format!("{:?}", Currency::ZMW), "ZMW");
}

#[test]
fn display_currency() {
    use stripe::Currency;
    assert_eq!(format!("{}", Currency::AED), "aed");
    assert_eq!(format!("{}", Currency::USD), "usd");
    assert_eq!(format!("{}", Currency::ZMW), "zmw");
}

#[test]
fn serialize_currency() {
    use stripe::Currency;
    assert_eq!(serde_json::to_string(&Currency::AED).unwrap(), "\"aed\"");
    assert_eq!(serde_json::to_string(&Currency::USD).unwrap(), "\"usd\"");
    assert_eq!(serde_json::to_string(&Currency::ZMW).unwrap(), "\"zmw\"");
}

#[test]
fn deserialize_currency() {
    use stripe::Currency;
    assert_eq!(serde_json::from_str::<Currency>("\"aed\"").unwrap(), Currency::AED);
    assert_eq!(serde_json::from_str::<Currency>("\"usd\"").unwrap(), Currency::USD);
    assert_eq!(serde_json::from_str::<Currency>("\"zmw\"").unwrap(), Currency::ZMW);
}

#[test]
fn serialize_range_query() {
    use stripe::{ListCustomers, RangeBounds, RangeQuery};

    let query = RangeQuery::Bounds(RangeBounds {
        gt: None,
        gte: Some(1501598702),
        lt: Some(1504233902),
        lte: None,
    });
    assert_eq!(urldecode(serde_qs::to_string(&query).unwrap()), "gte=1501598702&lt=1504233902");

    let mut params = ListCustomers::new();
    params.created = Some(RangeQuery::eq(1501598702));
    params.limit = Some(3);
    assert_eq!(urldecode(serde_qs::to_string(&params).unwrap()), "created=1501598702&limit=3");

    let mut params = ListCustomers::new();
    params.created = Some(RangeQuery::gte(1501598702));
    params.limit = Some(3);
    assert_eq!(urldecode(serde_qs::to_string(&params).unwrap()), "created[gte]=1501598702&limit=3");

    let mut params = ListCustomers::new();
    params.created = Some(query);
    params.limit = Some(3);
    let encoded = urldecode(serde_qs::to_string(&params).unwrap());
    assert_eq!(encoded, "created[gte]=1501598702&created[lt]=1504233902&limit=3");
}

fn urldecode(input: String) -> String {
    input.replace("%5B", "[").replace("%5D", "]")
}

#[test]
fn deserialize_payment_source_params() {
    use stripe::{PaymentSourceParams, SourceId, TokenId};

    let examples = [
        (
            json!("src_xyzABC123"),
            Some(PaymentSourceParams::Source("src_xyzABC123".parse::<SourceId>().unwrap())),
        ),
        (
            json!("tok_189g322eZvKYlo2CeoPw2sdy"),
            Some(PaymentSourceParams::Token(
                "tok_189g322eZvKYlo2CeoPw2sdy".parse::<TokenId>().unwrap(),
            )),
        ),
    ];

    for (value, expected) in &examples {
        let input = serde_json::to_string(value).unwrap();
        let parsed: Option<PaymentSourceParams> = serde_json::from_str(&input).ok();
        assert_eq!(json!(parsed), json!(expected));
    }
}

#[test]
fn serialize_payment_source_params() {
    use stripe::{PaymentSourceParams, SourceId, TokenId};

    let examples = [
        (
            PaymentSourceParams::Source("src_xyzABC123".parse::<SourceId>().unwrap()),
            json!("src_xyzABC123"),
        ),
        (
            PaymentSourceParams::Token("tok_189g322eZvKYlo2CeoPw2sdy".parse::<TokenId>().unwrap()),
            json!("tok_189g322eZvKYlo2CeoPw2sdy"),
        ),
    ];

    for (params, expected) in &examples {
        let value = serde_json::to_value(params).unwrap();
        assert_eq!(&value, expected);
    }
}

#[test]
fn deserialize_customer_with_card() {
    use stripe::Customer;

    let example = json!({
      "id": "cus_1234",
      "object": "customer",
      "account_balance": 0,
      "created": 1542631579,
      "currency": null,
      "default_source": "card_ABCD",
      "delinquent": false,
      "description": "Customer for green.tea@example.com",
      "discount": null,
      "email": null,
      "invoice_prefix": "99999AA",
      "livemode": false,
      "metadata": {},
      "shipping": null,
      "sources": {
        "object": "list",
        "data": [
          {
            "id": "card_ABCD",
            "object": "card",
            "address_city": null,
            "address_country": null,
            "address_line1": null,
            "address_line1_check": null,
            "address_line2": null,
            "address_state": null,
            "address_zip": null,
            "address_zip_check": null,
            "brand": "American Express",
            "country": "US",
            "customer": "cus_1234",
            "cvc_check": null,
            "dynamic_last4": null,
            "exp_month": 11,
            "exp_year": 2019,
            "fingerprint": "ffff9999ffff9999",
            "funding": "credit",
            "last4": "4242",
            "metadata": {},
            "name": null,
            "tokenization_method": null
          }
        ],
        "has_more": false,
        "total_count": 1,
        "url": "/v1/customers/cus_1234/sources"
      },
      "subscriptions": {
        "object": "list",
        "data": [],
        "has_more": false,
        "total_count": 0,
        "url": "/v1/customers/cus_1234/subscriptions"
      },
      "tax_info": null,
      "tax_info_verification": null
    });
    let result = serde_json::from_value::<Customer>(example);
    assert!(result.is_ok(), "expected ok; was {:?}", result);
}

#[test]
fn deserialize_customer_with_source() {
    use stripe::Customer;

    let example = json!({
      "id": "cus_5678",
      "object": "customer",
      "account_balance": 0,
      "created": 1538150891,
      "currency": null,
      "default_source": "src_EFGH",
      "delinquent": false,
      "description": null,
      "discount": null,
      "email": null,
      "invoice_prefix": "00AA00AA",
      "livemode": false,
      "metadata": {},
      "shipping": null,
      "sources": {
        "object": "list",
        "data": [
          {
            "id": "src_EFGH",
            "object": "source",
            "amount": null,
            "card": {
              "exp_month": 9,
              "exp_year": 2019,
              "brand": "Visa",
              "country": "US",
              "fingerprint": "AAAA1111aaaa2222",
              "funding": "credit",
              "last4": "4242",
              "three_d_secure": "optional",
              "name": null,
              "address_line1_check": null,
              "address_zip_check": null,
              "cvc_check": null,
              "tokenization_method": null,
              "dynamic_last4": null
            },
            "client_secret": "src_client_secret_SUPERSECRETTECH",
            "created": 1538150891,
            "currency": null,
            "customer": "cus_5678",
            "flow": "none",
            "livemode": false,
            "metadata": {},
            "owner": {
              "address": null,
              "email": null,
              "name": null,
              "phone": null,
              "verified_address": null,
              "verified_email": null,
              "verified_name": null,
              "verified_phone": null
            },
            "statement_descriptor": "My Company",
            "status": "chargeable",
            "type": "card",
            "usage": "reusable"
          }
        ],
        "has_more": false,
        "total_count": 1,
        "url": "/v1/customers/cus_5678/sources"
      },
      "subscriptions": {
        "object": "list",
        "data": [],
        "has_more": false,
        "total_count": 0,
        "url": "/v1/customers/cus_5678/subscriptions"
      },
      "tax_info": null,
      "tax_info_verification": null
    });
    let result = serde_json::from_value::<Customer>(example);
    assert!(result.is_ok(), "expected ok; was {:?}", result);
}

#[test]
#[cfg(feature = "events")]
fn deserialize_checkout_event() {
    use stripe::Event;

    let example = json!({
      "created": 1326853478,
      "livemode": false,
      "id": "evt_00000000000000",
      "type": "checkout.session.completed",
      "object": "event",
      "request": null,
      "pending_webhooks": 1,
      "api_version": "2019-05-16",
      "data": {
        "object": {
          "id": "cs_00000000000000",
          "created": 1649316731,
          "expires_at": 1649316731,
          "shipping_options": [],
          "custom_fields": [],
          "custom_text": {},
          "automatic_tax": {
              "enabled": false,
              "created": 1649316731,
          },
          "object": "checkout.session",
          "billing_address_collection": null,
          "cancel_url": "https://example.com/cancel",
          "client_reference_id": null,
          "customer": null,
          "customer_email": null,
          "display_items": [
            {
              "amount": 1500,
              "currency": "usd",
              "custom": {
                "description": "Comfortable cotton t-shirt",
                "images": null,
                "name": "T-shirt"
              },
              "quantity": 2,
              "type": "custom"
            }
          ],
          "livemode": false,
          "locale": null,
          "mode": "payment",
          "payment_intent": "pi_00000000000000",
          "payment_method_types": [
            "card"
          ],
          "payment_status": "paid",
          "submit_type": null,
          "subscription": null,
          "success_url": "https://example.com/success"
        }
      }
    });
    let result = serde_json::from_value::<Event>(example);
    assert!(result.is_ok(), "expected ok; was {:?}", result);
}
