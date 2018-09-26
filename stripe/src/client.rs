use error::{Error, ErrorObject, RequestError};
use hyper;
use hyper::client::RequestBuilder;
use hyper::header::{Authorization, Basic, ContentType, Headers};
use hyper::net::HttpsConnector;
use serde;
use serde_json as json;
use serde_qs as qs;
use std::io::Read;

#[derive(Clone, Default)]
pub struct Params {
    pub stripe_account: Option<String>,
}

// TODO: #[derive(Clone)]
pub struct Client {
    client: hyper::Client,
    secret_key: String,
    params: Params,
}

// TODO: With Hyper 0.11.x, hyper::Client implements clone, and we can just derive this
impl Clone for Client {
    fn clone(&self) -> Self {
        let mut client = Client::new(self.secret_key.as_str());
        client.params = self.params.clone();
        client
    }
}

impl Client {
    fn url(path: &str) -> String {
        format!("https://api.stripe.com/v1/{}", &path[1..])
    }

    #[cfg(feature = "with-rustls")]
    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        use hyper_rustls::TlsClient;

        let tls = TlsClient::new();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
        Client {
            client: client,
            secret_key: secret_key.into(),
            params: Params::default(),
        }
    }

    #[cfg(feature = "with-openssl")]
    pub fn new<Str: Into<String>>(secret_key: Str) -> Client {
        use hyper_openssl::OpensslClient;

        let tls = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = hyper::Client::with_connector(connector);
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

    pub fn post<T: serde::de::DeserializeOwned, P: serde::Serialize>(&self, path: &str, params: P) -> Result<T, Error> {
        let url = Client::url(path);
        let body = qs::to_string(&params)?;
        let request = self.client.post(&url).headers(self.headers()).body(&body);
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

    fn headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: self.secret_key.clone(),
            password: None,
        }));
        headers.set(ContentType::form_url_encoded());
        if let Some(ref account) = self.params.stripe_account {
            headers.set_raw("Stripe-Account", vec![account.as_bytes().to_vec()]);
        }
        headers
    }
}

fn send<T: serde::de::DeserializeOwned>(request: RequestBuilder) -> Result<T, Error> {
    let mut response = request.send()?;
    let mut body = String::with_capacity(4096);
    response.read_to_string(&mut body)?;

    let status = response.status_raw().0;
    match status {
        200...299 => {}
        _ => {
            let mut err = json::from_str(&body).unwrap_or_else(|err| {
                let mut req = ErrorObject { error: RequestError::default() };
                req.error.message = Some(format!("failed to deserialize error: {}", err));
                req
            });
            err.error.http_status = status;
            return Err(Error::from(err.error));
        }
    }

    json::from_str(&body).map_err(|err| Error::from(err))
}
