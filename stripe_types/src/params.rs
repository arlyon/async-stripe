use std::collections::HashMap;

use serde::Serialize;

use crate::{AccountId, ApiVersion, ApplicationId};

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
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
}
