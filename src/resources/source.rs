// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, SourceId, TokenId};
use crate::params::{Expand, Metadata, Object, Timestamp};
use crate::resources::{Address, BillingDetails, Currency};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Source".
///
/// For more details see [https://stripe.com/docs/api/sources/object](https://stripe.com/docs/api/sources/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    /// Unique identifier for the object.
    pub id: SourceId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<SourceTypeAchCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<SourceTypeAchDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<SourceTypeAlipay>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<SourceTypeBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SourceTypeCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<SourceTypeCardPresent>,

    /// The client secret of the source.
    ///
    /// Used for client-side retrieval using a publishable key.
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<SourceCodeVerificationFlow>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of the customer to which this source is attached.
    ///
    /// This will not be present when the source has not been attached to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<SourceTypeEps>,

    /// The authentication `flow` of the source.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: SourceFlow,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<SourceTypeGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<SourceTypeIdeal>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<SourceTypeMultibanco>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<SourceOwner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<SourceTypeP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<SourceReceiverFlow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<SourceRedirectFlow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SourceTypeSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<SourceTypeSofort>,

    /// Extra information about a source.
    ///
    /// This will appear on your customer's statement every time you charge the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    ///
    /// Only `chargeable` sources can be used to create a charge.
    pub status: SourceStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<SourceTypeThreeDSecure>,

    /// The `type` of the source.
    ///
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
    /// An additional hash is included on the source with a name matching this value.
    /// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[serde(rename = "type")]
    pub type_: SourceType,

    /// Either `reusable` or `single_use`.
    ///
    /// Whether this source should be reusable or not.
    /// Some source types may or may not be reusable by construction, while others may leave the option at creation.
    /// If an incompatible value is passed, an error will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SourceUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<SourceTypeWechat>,
}

impl Source {
    /// Creates a new source object.
    pub fn create(client: &Client, params: CreateSource<'_>) -> Response<Source> {
        client.post_form("/sources", &params)
    }

    /// Retrieves an existing source object.
    ///
    /// Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.
    pub fn retrieve(client: &Client, id: &SourceId, expand: &[&str]) -> Response<Source> {
        client.get_query(&format!("/sources/{}", id), &Expand { expand })
    }

    /// Updates the specified source by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request accepts the `metadata` and `owner` as arguments.
    /// It is also possible to update type specific information for selected payment methods.
    /// Please refer to our [payment method guides](https://stripe.com/docs/sources) for more detail.
    pub fn update(client: &Client, id: &SourceId, params: UpdateSource<'_>) -> Response<Source> {
        client.post_form(&format!("/sources/{}", id), &params)
    }
}

impl Object for Source {
    type Id = SourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "source"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceCodeVerificationFlow {
    /// The number of attempts remaining to authenticate the source object with a verification code.
    pub attempts_remaining: i64,

    /// The status of the code verification, either `pending` (awaiting verification, `attempts_remaining` should be greater than 0), `succeeded` (successful verification) or `failed` (failed verification, cannot be verified anymore as `attempts_remaining` should be 0).
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceOwner {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Verified owner's address.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_address: Option<Address>,

    /// Verified owner's email address.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_email: Option<String>,

    /// Verified owner's full name.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,

    /// Verified owner's phone number (including extension).
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_phone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceReceiverFlow {
    /// The address of the receiver source.
    ///
    /// This is the value that should be communicated to the customer to send their funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The total amount that was charged by you.
    ///
    /// The amount charged is expressed in the source's currency.
    pub amount_charged: i64,

    /// The total amount received by the receiver source.
    ///
    /// `amount_received = amount_returned + amount_charged` is true at all time.
    /// The amount received is expressed in the source's currency.
    pub amount_received: i64,

    /// The total amount that was returned to the customer.
    ///
    /// The amount returned is expressed in the source's currency.
    pub amount_returned: i64,

    /// Type of refund attribute method, one of `email`, `manual`, or `none`.
    pub refund_attributes_method: String,

    /// Type of refund attribute status, one of `missing`, `requested`, or `available`.
    pub refund_attributes_status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceRedirectFlow {
    /// The failure reason for the redirect, either `user_abort` (the customer aborted or dropped out of the redirect flow), `declined` (the authentication failed or the transaction was declined), or `processing_error` (the redirect failed due to a technical error).
    ///
    /// Present only if the redirect status is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<SourceRedirectFlowFailureReason>,

    /// The URL you provide to redirect the customer to after they authenticated their payment.
    pub return_url: String,

    /// The status of the redirect, either `pending` (ready to be used by your customer to authenticate the transaction), `succeeded` (succesful authentication, cannot be reused) or `not_required` (redirect should not be used) or `failed` (failed authentication, cannot be reused).
    pub status: SourceRedirectFlowStatus,

    /// The URL provided to you to redirect a customer to as part of a `redirect` authentication flow.
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAchCreditTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_routing_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAchDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAlipay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeCardPresent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_preferred_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_response_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvm_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_customer_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_transaction_certificate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_device_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_entry_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_verification_results: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status_information: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeEps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeGiropay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeMultibanco {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_iban: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeSofort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeThreeDSecure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,

    #[serde(default)]
    pub authenticated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeWechat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepay_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

/// The parameters for `Source::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSource<'a> {
    /// Amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The `Customer` to whom the original source is attached to.
    ///
    /// Must be set when the original source is not a `Source` (e.g., `Card`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The authentication `flow` of the source to create.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    /// It is generally inferred unless a type supports multiple flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<SourceFlow>,

    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<SourceMandateParams>,

    /// A set of key-value pairs that you can attach to a source object.
    ///
    /// It can be useful for storing additional information about the source in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The source to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_source: Option<&'a str>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<BillingDetails>,

    /// Optional parameters for the receiver flow.
    ///
    /// Can be set only if the source is a receiver (`flow` is `receiver`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<CreateSourceReceiver>,

    /// Parameters required for the redirect flow.
    ///
    /// Required if the source is authenticated by a redirect (`flow` is `redirect`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreateSourceRedirect>,

    /// An arbitrary string to be displayed on your customer's statement.
    ///
    /// As an example, if your website is `RunClub` and the item you're charging for is a race ticket, you may want to specify a `statement_descriptor` of `RunClub 5K race ticket.` While many payment types will display this information, some may not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,

    /// An optional token used to create the source.
    ///
    /// When passed, token properties will override source parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<TokenId>,

    /// The `type` of the source to create.
    ///
    /// Required unless `customer` and `original_source` are specified (see the [Shared card Sources](https://stripe.com/docs/sources/connect#shared-card-sources) guide).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SourceUsage>,
}

impl<'a> CreateSource<'a> {
    pub fn new() -> Self {
        CreateSource {
            amount: Default::default(),
            currency: Default::default(),
            customer: Default::default(),
            expand: Default::default(),
            flow: Default::default(),
            mandate: Default::default(),
            metadata: Default::default(),
            original_source: Default::default(),
            owner: Default::default(),
            receiver: Default::default(),
            redirect: Default::default(),
            statement_descriptor: Default::default(),
            token: Default::default(),
            type_: Default::default(),
            usage: Default::default(),
        }
    }
}

/// The parameters for `Source::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSource<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<SourceMandateParams>,

    /// A set of key-value pairs that you can attach to a source object.
    ///
    /// It can be useful for storing additional information about the source in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<BillingDetails>,
}

impl<'a> UpdateSource<'a> {
    pub fn new() -> Self {
        UpdateSource {
            expand: Default::default(),
            mandate: Default::default(),
            metadata: Default::default(),
            owner: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceReceiver {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_attributes_method: Option<SourceRefundNotificationMethod>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceRedirect {
    pub return_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMandateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<SourceAcceptanceParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<SourceMandateInterval>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<SourceMandateNotificationMethod>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceAcceptanceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<SourceAcceptanceOfflineParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<SourceAcceptanceOnlineParams>,

    pub status: SourceAcceptanceParamsStatus,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<SourceAcceptanceParamsType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceAcceptanceOfflineParams {
    pub contact_email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceAcceptanceOnlineParams {
    pub date: Timestamp,

    pub ip: String,

    pub user_agent: String,
}

/// An enum representing the possible values of an `SourceAcceptanceParams`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceAcceptanceParamsStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl SourceAcceptanceParamsStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceAcceptanceParamsStatus::Accepted => "accepted",
            SourceAcceptanceParamsStatus::Pending => "pending",
            SourceAcceptanceParamsStatus::Refused => "refused",
            SourceAcceptanceParamsStatus::Revoked => "revoked",
        }
    }
}

impl AsRef<str> for SourceAcceptanceParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceAcceptanceParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SourceAcceptanceParams`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceAcceptanceParamsType {
    Offline,
    Online,
}

impl SourceAcceptanceParamsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceAcceptanceParamsType::Offline => "offline",
            SourceAcceptanceParamsType::Online => "online",
        }
    }
}

impl AsRef<str> for SourceAcceptanceParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceAcceptanceParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Source`'s `flow` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
}

impl SourceFlow {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceFlow::CodeVerification => "code_verification",
            SourceFlow::None => "none",
            SourceFlow::Receiver => "receiver",
            SourceFlow::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for SourceFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SourceMandateParams`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}

impl SourceMandateInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceMandateInterval::OneTime => "one_time",
            SourceMandateInterval::Scheduled => "scheduled",
            SourceMandateInterval::Variable => "variable",
        }
    }
}

impl AsRef<str> for SourceMandateInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SourceMandateParams`'s `notification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}

impl SourceMandateNotificationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceMandateNotificationMethod::DeprecatedNone => "deprecated_none",
            SourceMandateNotificationMethod::Email => "email",
            SourceMandateNotificationMethod::Manual => "manual",
            SourceMandateNotificationMethod::None => "none",
            SourceMandateNotificationMethod::StripeEmail => "stripe_email",
        }
    }
}

impl AsRef<str> for SourceMandateNotificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SourceRedirectFlow`'s `failure_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRedirectFlowFailureReason {
    Declined,
    ProcessingError,
    UserAbort,
}

impl SourceRedirectFlowFailureReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceRedirectFlowFailureReason::Declined => "declined",
            SourceRedirectFlowFailureReason::ProcessingError => "processing_error",
            SourceRedirectFlowFailureReason::UserAbort => "user_abort",
        }
    }
}

impl AsRef<str> for SourceRedirectFlowFailureReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceRedirectFlowFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SourceRedirectFlow`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRedirectFlowStatus {
    Failed,
    NotRequired,
    Pending,
    Succeeded,
}

impl SourceRedirectFlowStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceRedirectFlowStatus::Failed => "failed",
            SourceRedirectFlowStatus::NotRequired => "not_required",
            SourceRedirectFlowStatus::Pending => "pending",
            SourceRedirectFlowStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for SourceRedirectFlowStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceRedirectFlowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateSourceReceiver`'s `refund_attributes_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRefundNotificationMethod {
    Email,
    Manual,
    None,
}

impl SourceRefundNotificationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceRefundNotificationMethod::Email => "email",
            SourceRefundNotificationMethod::Manual => "manual",
            SourceRefundNotificationMethod::None => "none",
        }
    }
}

impl AsRef<str> for SourceRefundNotificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceRefundNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Source`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Canceled,
    Chargeable,
    Consumed,
    Failed,
    Pending,
}

impl SourceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceStatus::Canceled => "canceled",
            SourceStatus::Chargeable => "chargeable",
            SourceStatus::Consumed => "consumed",
            SourceStatus::Failed => "failed",
            SourceStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for SourceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Source`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

impl SourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceType::AchCreditTransfer => "ach_credit_transfer",
            SourceType::AchDebit => "ach_debit",
            SourceType::Alipay => "alipay",
            SourceType::Bancontact => "bancontact",
            SourceType::Card => "card",
            SourceType::CardPresent => "card_present",
            SourceType::Eps => "eps",
            SourceType::Giropay => "giropay",
            SourceType::Ideal => "ideal",
            SourceType::Multibanco => "multibanco",
            SourceType::P24 => "p24",
            SourceType::SepaDebit => "sepa_debit",
            SourceType::Sofort => "sofort",
            SourceType::ThreeDSecure => "three_d_secure",
            SourceType::Wechat => "wechat",
        }
    }
}

impl AsRef<str> for SourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Source`'s `usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceUsage {
    Reusable,
    SingleUse,
}

impl SourceUsage {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceUsage::Reusable => "reusable",
            SourceUsage::SingleUse => "single_use",
        }
    }
}

impl AsRef<str> for SourceUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
