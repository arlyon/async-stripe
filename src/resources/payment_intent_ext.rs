use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::params::{Expandable, Metadata, SearchList};
use crate::resources::{Currency, PaymentSource, Shipping};
use crate::{PaymentIntent, PaymentIntentCancellationReason};

impl PaymentIntent {
    /// Confirm that customer intends to pay with current or provided source. Upon confirmation, the PaymentIntent will attempt to initiate a payment.
    ///
    /// For more details see <https://stripe.com/docs/api/payment_intents/confirm>.
    pub fn confirm(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentConfirmParams<'_>,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}/confirm", payment_intent_id), params)
    }

    /// Capture the funds of an existing uncaptured PaymentIntent where required_action="requires_capture".
    ///
    /// For more details see <https://stripe.com/docs/api/payment_intents/capture>.
    pub fn capture(
        client: &Client,
        payment_intent_id: &str,
        params: CapturePaymentIntent,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}/capture", payment_intent_id), params)
    }

    /// A PaymentIntent object can be canceled when it is in one of these statuses: requires_source, requires_capture, requires_confirmation, requires_source_action.
    ///
    /// For more details see <https://stripe.com/docs/api/payment_intents/cancel>.
    pub fn cancel(
        client: &Client,
        payment_intent_id: &str,
        params: CancelPaymentIntent,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}/cancel", payment_intent_id), params)
    }

    /// Searches for a payment intent.
    ///
    /// For more details see <https://stripe.com/docs/api/payment_intents/search>.
    pub fn search(
        client: &Client,
        params: PaymentIntentSearchParams,
    ) -> Response<SearchList<PaymentIntent>> {
        client.get_query("/payment_intents/search", params)
    }
}
/// The resource representing a Stripe PaymentError object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error>.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentError {
    #[serde(rename = "type")]
    pub payment_error_type: PaymentErrorType,
    pub charge: Option<String>,
    pub code: Option<String>,
    pub decline_code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub source: Option<Expandable<PaymentSource>>,
}

/// The resource representing a Stripe PaymentErrorType object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error-type>.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum PaymentErrorType {
    #[serde(rename = "api_error")]
    Api,
    #[serde(rename = "api_connection_error")]
    Connection,
    #[serde(rename = "authentication_error")]
    Authentication,
    #[serde(rename = "card_error")]
    Card,
    #[serde(rename = "idempotency_error")]
    Idempotency,
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,
    #[serde(rename = "rate_limit_error")]
    RateLimit,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

// TODO: This might be moved to `PaymentSourceType` if we determine
//       that all of the variants are _always_ the same.
//
//       In that case this can be replaced with a deprecated type alias.
/// Represents the way a `PaymentIntent` needs to be fulfilled.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentMethodType {
    /// This `PaymentIntent` needs to be fulfilled through credit card payment.
    Card,
    /// This `PaymentIntent` needs to be fulfilled through an
    /// [iDeal](https://stripe.com/docs/payments/ideal) payment.
    Ideal,
    /// This `PaymentIntent` needs to be fulfilled through a
    /// [Sepa Direct Debit](https://stripe.com/docs/payments/sepa-debit) payment.
    SepaDebit,
}

/// The resource representing a Stripe CaptureMethod object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/object#payment_intent_object-capture_method>.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}
/// The resource representing a Stripe ConfirmationMethod object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/object#payment_intent_object-confirmation_method>.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMethod {
    Secret,
    Publishable,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentNextActionType {
    RedirectToUrl,
    UseStripeSdk,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// The set of parameters that can be used when updating a payment_intent object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/update>
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentUpdateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The set of parameters that can be used when confirming a payment_intent object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/confirm>
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentConfirmParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
}

/// The set of parameters that can be used when capturing a payment_intent object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/capture>
#[derive(Clone, Debug, Default, Serialize)]
pub struct CapturePaymentIntent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to_capture: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
}

/// The set of parameters that can be used when canceling a payment_intent object.
///
/// For more details see <https://stripe.com/docs/api/payment_intents/cancel>
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelPaymentIntent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<PaymentIntentCancellationReason>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> PaymentIntentSearchParams<'a> {
    pub fn new() -> PaymentIntentSearchParams<'a> {
        PaymentIntentSearchParams { query: String::new(), limit: None, page: None, expand: &[] }
    }
}
