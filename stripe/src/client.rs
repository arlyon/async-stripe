use error::{Error, ErrorObject, RequestError};
use reqwest;
use reqwest::RequestBuilder;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, AUTHORIZATION,
};
use serde;
use serde_json;
use serde_qs;
use std::io::Read;

#[derive(Clone, Default)]
pub struct Params {
    pub stripe_account: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    secret_key: String,
    params: Params,
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://api.stripe.com/v1/{}", &path[1..])
    }

    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        let client = reqwest::Client::new();
        Client {
            client: client,
            secret_key: secret_key.into(),
            params: Params::default(),
        }
    }

    /// Clones a new client with different params.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand params while using the same secret key.
    pub fn with(&self, params: Params) -> Client {
        let mut client = self.clone();
        client.params = params;
        client
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Params{stripe_account: "acct_ABC", ..})`.
    pub fn set_stripe_account<Str: Into<String>>(&mut self, account_id: Str) {
        self.params.stripe_account = Some(account_id.into());
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let request = self.client.get(&url).headers(self.headers());
        send(request)
    }

    pub fn post<T: serde::de::DeserializeOwned, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Result<T, Error> {
        let url = Client::url(path);
        let request = self.client.post(&url).headers(self.headers());
        let request = with_querystring(request, &form)?;
        send(request)
    }

    pub fn post_empty<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let request = self.client.post(&url).headers(self.headers());
        send(request)
    }

    pub fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = Client::url(path);
        let request = self.client.delete(&url).headers(self.headers());
        send(request)
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap(),
        );
        if let Some(ref account) = self.params.stripe_account {
            headers.insert(
                HeaderName::from_static("stripe-account"),
                HeaderValue::from_str(account).unwrap(),
            );
        }
        if let Some(ref client_id) = self.params.client_id {
            headers.insert(
                HeaderName::from_static("client-id"),
                HeaderValue::from_str(client_id).unwrap(),
            );
        }
        headers
    }
}

/// Serialize the form content using `serde_qs` instead of `serde_urlencoded`
///
/// See https://github.com/seanmonstar/reqwest/issues/274
fn with_querystring<T: serde::Serialize>(request: RequestBuilder, form: &T) -> Result<RequestBuilder, Error> {
    let key = reqwest::header::CONTENT_TYPE;
    let value = reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded");
    let body = serde_qs::to_string(form)?;
    Ok(request.header(key, value).body(body))
}

fn send<T: serde::de::DeserializeOwned>(request: RequestBuilder) -> Result<T, Error> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(4096);
    response.read_to_string(&mut body)?;

    let status = response.status();
    if !status.is_success() {
        let mut err = serde_json::from_str(&body).unwrap_or_else(|err| {
            let mut req = ErrorObject {
                error: RequestError::default(),
            };
            req.error.message = Some(format!("failed to deserialize error: {}", err));
            req
        });
        err.error.http_status = status.as_u16();
        return Err(Error::from(err.error));
    }

    serde_json::from_str(&body).map_err(|err| Error::from(err))
}

#[cfg(test)]
mod tests {
    use ::{Client, CustomerParams};
    use std::collections::HashMap;
    use super::with_querystring;

    #[test]
    fn serialize_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("any".to_string(), "thing".to_string());
        let form = CustomerParams {
            email: Some("jdoe@example.org"),
            metadata: Some(metadata),
            // ...
            source: None,
            default_source: None,
            account_balance: None,
            business_vat_id: None,
            coupon: None,
            description: None,
            shipping: None,
        };
        let url = Client::url("/");
        let http = reqwest::Client::new();
        let result = with_querystring(http.post(&url), &form).and_then(|x| Ok(x.build()?));
        assert!(result.is_ok(), "Failed to build request: {:?}", result);
        if let Ok(request) = result {
            let body = format!("{:?}", request.body());
            assert_eq!(body, "Some(Body { kind: b\"email=jdoe%40example.org&metadata[any]=thing\" })");
        }
    }
}