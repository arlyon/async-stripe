extern crate hyper;
extern crate serde_json as json;
extern crate serde_qs as qs;

use params::to_snakecase;
use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// An error encountered when communicating with the Stripe API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by Stripe.
    Stripe(RequestError),
    /// A networking error communicating with the Stripe server.
    Http(hyper::Error),
    /// An error reading the response body.
    Io(io::Error),
    /// An error converting between wire format and Rust types.
    Conversion(Box<error::Error + Send>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))?;
        match *self {
            Error::Stripe(ref err) => write!(f, ": {}", err),
            Error::Http(ref err) => write!(f, ": {}", err),
            Error::Io(ref err) => write!(f, ": {}", err),
            Error::Conversion(ref err) => write!(f, ": {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Stripe(_) => "error reported by stripe",
            Error::Http(_) => "error communicating with stripe",
            Error::Io(_) => "error reading response from stripe",
            Error::Conversion(_) => "error converting between wire format and Rust types",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Stripe(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Conversion(ref err) => Some(&**err),
        }
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Stripe(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<qs::Error> for Error {
    fn from(err: qs::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::Conversion(Box::new(err))
    }
}


/// The list of possible values for a RequestError's type.
#[derive(Debug, PartialEq, Deserialize)]
pub enum ErrorType {
    #[serde(skip_deserializing)]
    Unknown,

    #[serde(rename = "api_error")]
    Api,
    #[serde(rename = "api_connection_error")]
    Connection,
    #[serde(rename = "authentication_error")]
    Authentication,
    #[serde(rename = "card_error")]
    Card,
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,
    #[serde(rename = "rate_limit_error")]
    RateLimit,
    #[serde(rename = "validation_error")]
    Validation,
}

impl Default for ErrorType {
    fn default() -> Self {
        ErrorType::Unknown
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}Error", self)))
    }
}

/// The list of possible values for a RequestError's code.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    AccountAlreadyExists,
    AccountCountryInvalidAddress,
    AccountInvalid,
    AccountNumberInvalid,
    AlipayUpgradeRequired,
    AmountTooLarge,
    AmountTooSmall,
    ApiKeyExpired,
    BalanceInsufficient,
    BankAccountExists,
    BankAccountUnusable,
    BankAccountUnverified,
    BitcoinUpgradeRequired,
    CardDeclined,
    ChargeAlreadyCaptured,
    ChargeAlreadyRefunded,
    ChargeDisputed,
    ChargeExpiredForCapture,
    CountryUnsupported,
    CouponExpired,
    CustomerMaxSubscriptions,
    EmailInvalid,
    ExpiredCard,
    IncorrectAddress,
    IncorrectCvc,
    IncorrectNumber,
    IncorrectZip,
    InstantPayoutsUnsupported,
    InvalidCardType,
    InvalidChargeAmount,
    InvalidCvc,
    InvalidExpiryMonth,
    InvalidExpiryYear,
    InvalidNumber,
    InvalidSourceUsage,
    InvoiceNoCustomerLineItems,
    InvoiceNoSubscriptionLineItems,
    InvoiceNotEditable,
    InvoiceUpcomingNone,
    LivemodeMismatch,
    Missing,
    OrderCreationFailed,
    OrderRequiredSettings,
    OrderStatusInvalid,
    OrderUpstreamTimeout,
    OutOfInventory,
    ParameterInvalidEmpty,
    ParameterInvalidInteger,
    ParameterInvalidStringBlank,
    ParameterInvalidStringEmpty,
    ParameterMissing,
    ParameterUnknown,
    PaymentMethodUnactivated,
    PayoutsNotAllowed,
    PlatformApiKeyExpired,
    PostalCodeInvalid,
    ProcessingError,
    ProductInactive,
    RateLimit,
    ResourceAlreadyExists,
    ResourceMissing,
    RoutingNumberInvalid,
    SecretKeyRequired,
    SepaUnsupportedAccount,
    ShippingCalculationFailed,
    SkuInactive,
    StateUnsupported,
    TaxIdInvalid,
    TaxesCalculationFailed,
    TestmodeChargesOnly,
    TlsVersionUnsupported,
    TokenAlreadyUsed,
    TokenInUse,
    TransfersNotAllowed,
    UpstreamOrderCreationFailed,
    UrlInvalid,
    #[doc(hidden)] __NonExhaustive,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}", self)))
    }
}

/// An error reported by stripe in a request's response.
///
/// For more details see https://stripe.com/docs/api#errors.
#[derive(Debug, Default, Deserialize)]
pub struct RequestError {
    /// The HTTP status in the response.
    #[serde(skip_deserializing)]
    pub http_status: u16,

    /// The type of error returned.
    #[serde(rename = "type")]
    pub error_type: ErrorType,

    /// A human-readable message providing more details about the error.
    /// For card errors, these messages can be shown to end users.
    #[serde(default)]
    pub message: Option<String>,

    /// For card errors, a value describing the kind of card error that occured.
    pub code: Option<ErrorCode>,

    /// For card errors resulting from a bank decline, a string indicating the
    /// bank's reason for the decline if they provide one.
    pub decline_code: Option<String>,

    /// The ID of the failed charge, if applicable.
    pub charge: Option<String>,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.error_type, self.http_status)?;
        if let Some(ref message) = self.message {
            write!(f, ": {}", message)?;
        }
        Ok(())
    }
}

impl error::Error for RequestError {
    fn description(&self) -> &str {
        self.message.as_ref().map(|s| s.as_str()).unwrap_or(
            "request error",
        )
    }
}

#[doc(hidden)]
#[derive(Deserialize)]
pub struct ErrorObject {
    pub error: RequestError,
}

/// An error encountered when communicating with the Stripe API webhooks.
#[derive(Debug)]
pub enum WebhookError {
    BadHeader(ParseIntError),
    BadSignature,
    BadTimestamp(i64),
    BadParse(json::Error),
}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))?;
        match *self {
            WebhookError::BadHeader(ref err) => write!(f, ": {}", err),
            WebhookError::BadSignature => write!(f, "Signatures do not match"),
            WebhookError::BadTimestamp(ref err) => write!(f, ": {}", err),
            WebhookError::BadParse(ref err) => write!(f, ": {}", err),
        }
    }
}

impl error::Error for WebhookError {
    fn description(&self) -> &str {
        match *self {
            WebhookError::BadHeader(_) => "error parsing timestamp",
            WebhookError::BadSignature => "error comparing signatures",
            WebhookError::BadTimestamp(_) => "error comparing timestamps - over tolerance",
            WebhookError::BadParse(_) => "error parsing event object",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WebhookError::BadHeader(ref err) => Some(err),
            WebhookError::BadSignature => None,
            WebhookError::BadTimestamp(_) => None,
            WebhookError::BadParse(ref err) => Some(err),
        }
    }
}
