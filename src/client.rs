use crate::config::Response;
use crate::error::{Error, ErrorResponse, RequestError};
use crate::params::Headers;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use std::io::Read;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    secret_key: String,
    headers: Headers,
}

impl Client {
    pub fn new<S: Into<String>>(secret_key: S) -> Client {
        let client = reqwest::Client::new();
        Client { client, secret_key: secret_key.into(), headers: Headers::default() }
    }

    /// Clones a new client with different headers.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand headers while using the same secret key.
    pub fn with_headers(&self, headers: Headers) -> Client {
        let mut client = self.clone();
        client.headers = headers;
        client
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Headers{stripe_account: "acct_ABC", ..})`.
    pub fn set_stripe_account<S: Into<String>>(&mut self, account_id: S) {
        self.headers.stripe_account = Some(account_id.into());
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Response<T> {
        let url = Client::url(path);
        let request = self.client.get(&url).headers(self.headers());
        send(request)
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = Client::url_with_params(path, params)?;
        let request = self.client.get(&url).headers(self.headers());
        send(request)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned>(&self, path: &str) -> Response<T> {
        let url = Client::url(path);
        let request = self.client.delete(&url).headers(self.headers());
        send(request)
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = Client::url_with_params(path, params)?;
        let request = self.client.delete(&url).headers(self.headers());
        send(request)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned>(&self, path: &str) -> Response<T> {
        let url = Client::url(path);
        let request = self.client.post(&url).headers(self.headers());
        send(request)
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = Client::url(path);
        let request = self.client.post(&url).headers(self.headers());
        let request = with_form_urlencoded(request, &form)?;
        send(request)
    }

    fn url(path: &str) -> String {
        format!("https://api.stripe.com/v1/{}", &path[1..])
    }

    fn url_with_params<P: serde::Serialize>(path: &str, params: P) -> Result<String, Error> {
        let params = serde_qs::to_string(&params).map_err(Error::serialize)?;
        Ok(format!("https://api.stripe.com/v1/{}?{}", &path[1..], params))
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap(),
        );
        if let Some(account) = &self.headers.stripe_account {
            headers.insert(
                HeaderName::from_static("stripe-account"),
                HeaderValue::from_str(account).unwrap(),
            );
        }
        if let Some(client_id) = &self.headers.client_id {
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
fn with_form_urlencoded<T: serde::Serialize>(
    request: RequestBuilder,
    form: &T,
) -> Result<RequestBuilder, Error> {
    let body = serde_qs::to_string(form).map_err(Error::serialize)?;
    Ok(request
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body))
}

fn send<T: DeserializeOwned>(request: RequestBuilder) -> Response<T> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(4096);
    response.read_to_string(&mut body)?;

    let status = response.status();
    if !status.is_success() {
        let mut err = serde_json::from_str(&body).unwrap_or_else(|err| {
            let mut req = ErrorResponse { error: RequestError::default() };
            req.error.message = Some(format!("failed to deserialize error: {}", err));
            req
        });
        err.error.http_status = status.as_u16();
        return Err(Error::from(err.error));
    }

    serde_json::from_str(&body).map_err(Error::deserialize)
}

#[cfg(test)]
mod tests {
    use super::{with_form_urlencoded, Client};
    use crate::CustomerParams;
    use std::collections::HashMap;

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
        let result = with_form_urlencoded(http.post(&url), &form).and_then(|x| Ok(x.build()?));
        assert!(result.is_ok(), "Failed to build request: {:?}", result);
        if let Ok(request) = result {
            let body = format!("{:?}", request.body());
            assert_eq!(
                body,
                "Some(Body { kind: b\"email=jdoe%40example.org&metadata[any]=thing\" })"
            );
        }
    }
}
