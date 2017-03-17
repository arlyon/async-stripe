extern crate hyper;
extern crate serde_json as json;
extern crate serde_qs as query;

use std::error::Error as StandardError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    BadStatus { code: u16, body: String },

    // These imply the stripe client had an error
    Decode(json::Error),
    Encode(query::ser::Error),

    // These imply the stripe service had an error
    Http(hyper::Error),
    Io(io::Error),
}

#[derive(Debug)]
pub enum Blame {
    Client,
    Server,
}

impl Error {
    pub fn from_status(code: u16, body: String) -> Error {
        Error::BadStatus{code: code, body: body}
    }
    pub fn from_encode(err: query::ser::Error) -> Error {
        Error::Encode(err)
    }
    pub fn from_decode(err: json::Error) -> Error {
        Error::Decode(err)
    }
    pub fn from_http(err: hyper::Error) -> Error {
        Error::Http(err)
    }
    pub fn from_io(err: io::Error) -> Error {
        Error::Io(err)
    }

    pub fn blame(&self) -> Blame {
        match self {
            &Error::BadStatus { code, .. } => match code {
                400...499 => Blame::Client,
                500...599 => Blame::Server,
                _ => Blame::Client,
            },
            &Error::Decode(_) => Blame::Client,
            &Error::Encode(_) => Blame::Client,
            &Error::Http(_) => Blame::Server,
            &Error::Io(_) => Blame::Server,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            &Error::BadStatus { code, ref body } => format!("stripe: bad http status {}: {}", code, body),
            &Error::Decode(ref err) => format!("stripe: {}", err),
            &Error::Encode(ref err) => format!("stripe: {}", err),
            &Error::Http(ref err) => format!("stripe: {}", err),
            &Error::Io(ref err) => format!("stripe: {}", err),
        };

        write!(f, "{}", message)
    }
}

impl StandardError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::BadStatus { .. } => "bad http status",
            &Error::Decode(ref err) => err.description(),
            &Error::Encode(ref err) => err.description(),
            &Error::Http(ref err) => err.description(),
            &Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StandardError> {
        match self {
            &Error::BadStatus { .. } => None,
            &Error::Decode(ref err) => Some(err),
            &Error::Encode(ref err) => Some(err),
            &Error::Http(ref err) => Some(err),
            &Error::Io(ref err) => Some(err),
        }
    }
}
