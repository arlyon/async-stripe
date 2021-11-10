// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::resources::{BankAccount, Card, PaymentIntent, PaymentMethod, SetupIntent, Source};

/// The resource representing a Stripe "APIErrors".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiErrors {
    /// For card errors, the ID of the failed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// For card errors resulting from a card issuer decline, a short string indicating the [card issuer's reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,

    /// A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,

    /// A human-readable message providing more details about the error.
    ///
    /// For card errors, these messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// If the error is parameter-specific, the parameter related to the error.
    ///
    /// For example, you can use this to display a message near the correct form field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,

    /// If the error is specific to the type of payment method, the payment method type that had a problem.
    ///
    /// This field is only populated for invoice-related errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<SetupIntent>,

    /// The source object for errors returned on a request involving a source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApiErrorsSource>,

    /// The type of error returned.
    ///
    /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`.
    #[serde(rename = "type")]
    pub type_: ApiErrorsType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum ApiErrorsSource {
    BankAccount(BankAccount),
    Card(Card),
    Source(Source),
}

/// An enum representing the possible values of an `ApiErrors`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorsType {
    ApiError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
}

impl ApiErrorsType {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiErrorsType::ApiError => "api_error",
            ApiErrorsType::CardError => "card_error",
            ApiErrorsType::IdempotencyError => "idempotency_error",
            ApiErrorsType::InvalidRequestError => "invalid_request_error",
        }
    }
}

impl AsRef<str> for ApiErrorsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiErrorsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
