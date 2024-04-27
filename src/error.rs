use std::num::ParseIntError;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::params::to_snakecase;

/// An error encountered when communicating with the Stripe API.
#[derive(Debug, Error)]
pub enum StripeError {
    #[error("error reported by stripe: {0}")]
    Stripe(#[from] RequestError),
    #[error("error serializing or deserializing a querystring: {0}")]
    QueryStringSerialize(#[from] serde_path_to_error::Error<serde_qs::Error>),
    #[error("error serializing or deserializing a request")]
    JSONSerialize(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error("attempted to access an unsupported version of the api")]
    UnsupportedVersion,
    #[error("error communicating with stripe: {0}")]
    ClientError(String),
    #[error("timeout communicating with stripe")]
    Timeout,
}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for StripeError {
    fn from(err: hyper::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

impl From<http_types::Error> for StripeError {
    fn from(err: http_types::Error) -> StripeError {
        StripeError::ClientError(err.to_string())
    }
}

/// The list of possible values for a RequestError's type.
#[derive(Debug, PartialEq, Deserialize, Default)]
pub enum ErrorType {
    #[serde(skip_deserializing)]
    #[default]
    Unknown,
    #[serde(rename = "api_error")]
    Api,
    #[serde(rename = "api_connection_error")]
    Connection,
    #[serde(rename = "authentication_error")]
    Authentication,
    #[serde(rename = "card_error")]
    Card,
    #[serde(rename = "idempotency_error")]
    IdempotencyError,
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,
    #[serde(rename = "rate_limit_error")]
    RateLimit,
    #[serde(rename = "validation_error")]
    Validation,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}Error", self)))
    }
}

/// The list of possible values for a RequestError's code.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
    BankAccountVerificationFailed,
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
    IdempotencyKeyInUse,
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
    PaymentIntentUnexpectedState,
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
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}", self)))
    }
}

/// An error reported by stripe in a request's response.
///
/// For more details see <https://stripe.com/docs/api#errors>.
#[derive(Debug, Default, Deserialize, Error)]
#[error("{error_type} ({http_status}){}", message.as_ref().map(|msg| {
    format!(" with message: {msg:?}")
}).unwrap_or_default())]
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

/// The structure of the json body when an error is included in
/// the response from Stripe.
#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: RequestError,
}

/// An error encountered when communicating with the Stripe API webhooks.
#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("invalid key length")]
    BadKey,
    #[error("error parsing timestamp")]
    BadHeader(#[from] ParseIntError),
    #[error("error comparing signatures")]
    BadSignature,
    #[error("error comparing timestamps - over tolerance")]
    BadTimestamp(i64),
    #[error("error parsing event object")]
    BadParse(#[from] serde_json::Error),
}
