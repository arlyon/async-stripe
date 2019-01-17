use crate::params::to_snakecase;
use serde_derive::{Deserialize, Serialize};
use std::num::ParseIntError;

/// An error encountered when communicating with the Stripe API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by Stripe in the response body.
    Stripe(RequestError),
    /// A networking error communicating with the Stripe server.
    Http(reqwest::Error),
    /// An error reading the response body.
    Io(std::io::Error),
    /// An error serializing a request before it is sent to stripe.
    Serialize(Box<dyn std::error::Error + Send>),
    /// An error deserializing a response received from stripe.
    Deserialize(Box<dyn std::error::Error + Send>),
    /// Indicates an operation not supported (yet?) by this library.
    Unsupported(&'static str),
    /// An invariant has been violated. Either a bug in this library or Stripe
    Unexpected(&'static str),
}

impl Error {
    pub fn serialize<T>(err: T) -> Error
    where
        T: std::error::Error + Send + 'static,
    {
        Error::Serialize(Box::new(err))
    }

    pub fn deserialize<T>(err: T) -> Error
    where
        T: std::error::Error + Send + 'static,
    {
        Error::Deserialize(Box::new(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::error::Error::description(self))?;
        match *self {
            Error::Stripe(ref err) => write!(f, ": {}", err),
            Error::Http(ref err) => write!(f, ": {}", err),
            Error::Io(ref err) => write!(f, ": {}", err),
            Error::Serialize(ref err) => write!(f, ": {}", err),
            Error::Deserialize(ref err) => write!(f, ": {}", err),
            Error::Unsupported(msg) => write!(f, "{}", msg),
            Error::Unexpected(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Stripe(_) => "error reported by stripe",
            Error::Http(_) => "error communicating with stripe",
            Error::Io(_) => "error reading response from stripe",
            Error::Serialize(_) => "error serializing a request",
            Error::Deserialize(_) => "error deserializing a response",
            Error::Unsupported(_) => "an unsupported operation was attempted",
            Error::Unexpected(_) => "an unexpected error has occurred",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Stripe(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Serialize(ref err) => Some(&**err),
            Error::Deserialize(ref err) => Some(&**err),
            Error::Unsupported(_) => None,
            Error::Unexpected(_) => None,
        }
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Stripe(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
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

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}Error", self)))
    }
}

/// The list of possible values for a RequestError's code.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
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
    #[doc(hidden)]
    __NonExhaustive,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.error_type, self.http_status)?;
        if let Some(ref message) = self.message {
            write!(f, ": {}", message)?;
        }
        Ok(())
    }
}

impl std::error::Error for RequestError {
    fn description(&self) -> &str {
        self.message.as_ref().map(|s| s.as_str()).unwrap_or("request error")
    }
}

/// The structure of the json body when an error is included in
/// the response from Stripe.
#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: RequestError,
}

/// An error encountered when communicating with the Stripe API webhooks.
#[derive(Debug)]
pub enum WebhookError {
    /// The provided secret could not be parsed as a key
    /// (e.g. it may not be the correct size).
    BadKey,
    /// The event's headers are in an unexpected format.
    BadHeader(ParseIntError),
    /// The event signature could not be verified.
    BadSignature,
    /// The event signature's timestamp was too old.
    BadTimestamp(i64),
    /// An error deserializing an event received from stripe.
    BadParse(serde_json::Error),
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::error::Error::description(self))?;
        match *self {
            WebhookError::BadKey => Ok(()),
            WebhookError::BadHeader(ref err) => write!(f, ": {}", err),
            WebhookError::BadSignature => Ok(()),
            WebhookError::BadTimestamp(ref err) => write!(f, ": {}", err),
            WebhookError::BadParse(ref err) => write!(f, ": {}", err),
        }
    }
}

impl std::error::Error for WebhookError {
    fn description(&self) -> &str {
        match *self {
            WebhookError::BadKey => "invalid key length",
            WebhookError::BadHeader(_) => "error parsing timestamp",
            WebhookError::BadSignature => "error comparing signatures",
            WebhookError::BadTimestamp(_) => "error comparing timestamps - over tolerance",
            WebhookError::BadParse(_) => "error parsing event object",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            WebhookError::BadKey => None,
            WebhookError::BadHeader(ref err) => Some(err),
            WebhookError::BadSignature => None,
            WebhookError::BadTimestamp(_) => None,
            WebhookError::BadParse(ref err) => Some(err),
        }
    }
}
