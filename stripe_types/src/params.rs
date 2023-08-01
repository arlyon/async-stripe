use std::collections::HashMap;

use serde::{Deserializer, Serialize, Serializer};

use crate::{AccountId, ApiVersion, ApplicationId};

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AlwaysTrue;

impl serde::Serialize for AlwaysTrue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

impl<'de> serde::Deserialize<'de> for AlwaysTrue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bool_: bool = serde::Deserialize::deserialize(deserializer)?;
        if !bool_ {
            Err(serde::de::Error::custom("Expected value to always be `true`"))
        } else {
            Ok(AlwaysTrue)
        }
    }
}

impl ToString for AppInfo {
    /// Formats a plugin's 'App Info' into a string that can be added to the end of an User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn to_string(&self) -> String {
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => format!("{}/{} ({})", &self.name, a, b),
            (Some(a), None) => format!("{}/{}", &self.name, a),
            (None, Some(b)) => format!("{} ({})", &self.name, b),
            _ => self.name.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Headers {
    pub stripe_version: ApiVersion,
    pub user_agent: String,

    pub client_id: Option<ApplicationId>,
    pub stripe_account: Option<AccountId>,
}

impl Headers {
    pub fn to_array(&self) -> [(&str, Option<&str>); 4] {
        [
            ("Client-Id", self.client_id.as_deref()),
            ("Stripe-Account", self.stripe_account.as_deref()),
            ("Stripe-Version", Some(self.stripe_version.as_str())),
            ("User-Agent", Some(&self.user_agent)),
        ]
    }
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Copy, Clone, Debug, Serialize, Default)]
pub struct RangeBoundsTs {
    pub gt: Option<Timestamp>,
    pub gte: Option<Timestamp>,
    pub lt: Option<Timestamp>,
    pub lte: Option<Timestamp>,
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Copy, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum RangeQueryTs {
    Exact(Timestamp),
    Bounds(RangeBoundsTs),
}

impl RangeQueryTs {
    /// Filter results to exactly match a given value
    pub fn eq(value: Timestamp) -> Self {
        Self::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { gt: Some(value), ..Default::default() })
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { gte: Some(value), ..Default::default() })
    }

    /// Filter results to be before to a given value
    pub fn lt(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { lt: Some(value), ..Default::default() })
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: Timestamp) -> Self {
        Self::Bounds(RangeBoundsTs { lte: Some(value), ..Default::default() })
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn list() {
        use httpmock::Method::GET;
        use httpmock::MockServer;

        use crate::Client;
        use crate::{Customer, ListCustomers};

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

        let params = ListCustomers::new();
        let res = Customer::list(&client, &params).await.unwrap().paginate(params);

        println!("{:?}", res);

        let res2 = res.next(&client).await.unwrap();

        println!("{:?}", res2);

        first_item.assert_hits_async(1).await;
        next_item.assert_hits_async(1).await;
    }

    #[cfg(all(feature = "async", feature = "stream"))]
    #[tokio::test]
    async fn stream() {
        use futures_util::StreamExt;
        use httpmock::Method::GET;
        use httpmock::MockServer;

        use crate::Client;
        use crate::{Customer, ListCustomers};

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

        let params = ListCustomers::new();
        let res = Customer::list(&client, &params).await.unwrap().paginate(params);

        let stream = res.stream(&client).collect::<Vec<_>>().await;

        println!("{:#?}", stream);
        assert_eq!(stream.len(), 2);

        first_item.assert_hits_async(1).await;
        next_item.assert_hits_async(1).await;
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
        assert_eq!(
            urldecode(serde_qs::to_string(&params).unwrap()),
            "created[gte]=1501598702&limit=3"
        );

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
                PaymentSourceParams::Token(
                    "tok_189g322eZvKYlo2CeoPw2sdy".parse::<TokenId>().unwrap(),
                ),
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
    #[cfg(feature = "event")]
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
}
