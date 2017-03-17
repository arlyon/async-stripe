extern crate serde;
extern crate serde_json as json;
extern crate serde_qs as query;

use error::Error;
use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::header::{Authorization, Basic, ContentType, Headers};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use std::io::Read;

pub fn get<T: serde::Deserialize>(path: &str, key: &str) -> Result<T, Error> {
    let client = get_client();
    let url = get_url(path);
    let headers = get_headers(key);
    let request = client.get(&url).headers(headers);
    send(request)
}

pub fn post<T: serde::Deserialize, P: serde::Serialize>(path: &str, key: &str, params: P) -> Result<T, Error> {
    let client = get_client();
    let url = get_url(path);
    let headers = get_headers(key);
    let body = match query::to_string(&params) {
        Err(err) => { return Err(Error::from_encode(err)); },
        Ok(data) => data,
    };
    let request = client.post(&url).headers(headers).body(&body);
    send(request)
}

pub fn delete<T: serde::Deserialize>(path: &str, key: &str) -> Result<T, Error> {
    let client = get_client();
    let url = get_url(path);
    let headers = get_headers(key);
    let request = client.delete(&url).headers(headers);
    send(request)
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

fn get_client() -> Client {
    let ssl = OpensslClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    Client::with_connector(connector)
}

fn get_url(path: &str) -> String {
    String::from("https://api.stripe.com/v1") + path
}

fn get_headers(key: &str) -> Headers {
    let mut headers = Headers::new();
    headers.set(Authorization(Basic{username: key.to_owned(), password: None}));
    headers.set(ContentType::form_url_encoded());
    headers
}
