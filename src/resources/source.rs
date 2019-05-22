use crate::config::{Client, Response};
use crate::ids::{SourceId, TokenId};
use crate::params::{Metadata, Object, Timestamp};
use crate::resources::{Address, Currency};
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
    pub fn create(client: &Client, params: SourceParams<'_>) -> Response<Source> {
        client.post_form("/sources", params)
    }

    pub fn get(client: &Client, source_id: &SourceId) -> Response<Source> {
        client.get(&format!("/sources/{}", source_id))
    }

    pub fn update(
        client: &Client,
        source_id: &SourceId,
        params: SourceParams<'_>,
    ) -> Response<Source> {
        client.post_form(&format!("/source/{}", source_id), params)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedirectParams<'a> {
    return_url: &'a str,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OwnerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}

impl<'a> OwnerParams<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SourceParams<'a> {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<SourceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<SourceFlow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<OwnerParams<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<RedirectParams<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<TokenId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SourceUsage>,
}

impl<'a> SourceParams<'a> {
    pub fn new() -> Self {
        Default::default()
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

/// An enum representing the possible values of an `SourceRedirectFlow`'s `failure_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRedirectFlowFailureReason {
    Declined,
    ProcessingError,
    UserAbort,
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

/// An enum representing the possible values of an `Source`'s `usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceUsage {
    Reusable,
    SingleUse,
}
