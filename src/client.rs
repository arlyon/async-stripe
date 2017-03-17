extern crate serde;
extern crate serde_json as json;
extern crate serde_qs as query;

use error::Error;
use hyper::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::header::{Authorization, Basic, ContentType, Headers};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use std::io::Read;

pub struct Client {
    client: HttpClient,
    secret_key: String, // <-- not to be modified (b.c. Sync)
}

impl Client {
    pub fn new(secret_key: String) -> Client {
        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client{
            client: HttpClient::with_connector(connector),
            secret_key: secret_key,
        }
    }

    pub fn get<T: serde::Deserialize>(&self, path: &str) -> Result<T, Error> {
        let url = get_url(path);
        let request = self.client.get(&url).headers(self.headers());
        send(request)
    }

    pub fn post<T: serde::Deserialize, P: serde::Serialize>(&self, path: &str, params: P) -> Result<T, Error> {
        let url = get_url(path);
        let body = match query::to_string(&params) {
            Err(err) => { return Err(Error::from_encode(err)); },
            Ok(data) => data,
        };
        let request = self.client.post(&url).headers(self.headers()).body(&body);
        send(request)
    }

    pub fn delete<T: serde::Deserialize>(&self, path: &str) -> Result<T, Error> {
        let url = get_url(path);
        let request = self.client.delete(&url).headers(self.headers());
        send(request)
    }

    fn headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic{username: self.secret_key.to_owned(), password: None}));
        headers.set(ContentType::form_url_encoded());
        headers
    }
}

fn get_url(path: &str) -> String {
    String::from("https://api.stripe.com/v1") + path
}

fn send<T: serde::Deserialize>(request: RequestBuilder) -> Result<T, Error> {
    let mut response = match request.send() {
        Err(err) => { return Err(Error::from_http(err)); },
        Ok(data) => data,
    };

    // Read the body from the response
    let mut body_string = String::with_capacity(4096);
    match response.read_to_string(&mut body_string) {
        Err(err) => { return Err(Error::from_io(err)); },
        Ok(_) => {},
    }

    // Handle the returned status code
    let status_code = response.status_raw().0;
    match status_code {
        200...299 => { /* ok */ },
        // TODO: probably merge these branches...
        400...499 => { return Err(Error::from_status(status_code, body_string)); },
        500...599 => { return Err(Error::from_status(status_code, body_string)); },
        _ => { return Err(Error::from_status(status_code, body_string)); }
    }

    match json::from_str(&body_string) {
        Err(err) => Err(Error::from_decode(err)),
        Ok(data) => Ok(data),
    }
}
