// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, SourceId, TokenId};
use crate::params::{Expand, List, Metadata, Object, Timestamp};
use crate::resources::{
    Address, BillingDetails, Currency, Shipping, SourceRedirectFlowFailureReason,
    SourceRedirectFlowStatus, SourceStatus, SourceUsage,
};

/// The resource representing a Stripe "Source".
///
/// For more details see <https://stripe.com/docs/api/sources/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    /// Unique identifier for the object.
    pub id: SourceId,

    pub ach_credit_transfer: Box<Option<SourceTypeAchCreditTransfer>>,

    pub ach_debit: Box<Option<SourceTypeAchDebit>>,

    pub acss_debit: Box<Option<SourceTypeAcssDebit>>,

    pub alipay: Box<Option<SourceTypeAlipay>>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Box<Option<i64>>,

    pub au_becs_debit: Box<Option<SourceTypeAuBecsDebit>>,

    pub bancontact: Box<Option<SourceTypeBancontact>>,

    pub card: Box<Option<SourceTypeCard>>,

    pub card_present: Box<Option<SourceTypeCardPresent>>,

    /// The client secret of the source.
    ///
    /// Used for client-side retrieval using a publishable key.
    pub client_secret: String,

    pub code_verification: Box<Option<SourceCodeVerificationFlow>>,

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
    pub customer: Box<Option<String>>,

    pub eps: Box<Option<SourceTypeEps>>,

    /// The authentication `flow` of the source.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: SourceFlow,

    pub giropay: Box<Option<SourceTypeGiropay>>,

    pub ideal: Box<Option<SourceTypeIdeal>>,

    pub klarna: Box<Option<SourceTypeKlarna>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    pub multibanco: Box<Option<SourceTypeMultibanco>>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Box<Option<SourceOwner>>,

    pub p24: Box<Option<SourceTypeP24>>,

    pub receiver: Box<Option<SourceReceiverFlow>>,

    pub redirect: Box<Option<SourceRedirectFlow>>,

    pub sepa_debit: Box<Option<SourceTypeSepaDebit>>,

    pub sofort: Box<Option<SourceTypeSofort>>,

    pub source_order: Box<Option<SourceOrder>>,

    /// Extra information about a source.
    ///
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Box<Option<String>>,

    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    ///
    /// Only `chargeable` sources can be used to create a charge.
    pub status: SourceStatus,

    pub three_d_secure: Box<Option<SourceTypeThreeDSecure>>,

    /// The `type` of the source.
    ///
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
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

    pub wechat: Box<Option<SourceTypeWechat>>,
}

impl Source {
    /// List source transactions for a given source.
    pub fn list(client: &Client, params: ListSources<'_>) -> Response<List<Source>> {
        client.get_query("/sources/{source}/source_transactions", &params)
    }

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
pub struct SourceOrder {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The email address of the customer placing the order.
    pub email: Box<Option<String>>,

    /// List of items constituting the order.
    pub items: Box<Option<Vec<SourceOrderItem>>>,

    pub shipping: Box<Option<Shipping>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceOrderItem {
    /// The amount (price) for this order item.
    pub amount: Box<Option<i64>>,

    /// This currency of this order item.
    ///
    /// Required when `amount` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Human-readable description for this order item.
    pub description: Box<Option<String>>,

    /// The ID of the associated object for this line item.
    ///
    /// Expandable if not null (e.g., expandable to a SKU).
    pub parent: Box<Option<String>>,

    /// The quantity of this order item.
    ///
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    pub quantity: Box<Option<u64>>,

    /// The type of this order item.
    ///
    /// Must be `sku`, `tax`, or `shipping`.
    #[serde(rename = "type")]
    pub type_: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceOwner {
    /// Owner's address.
    pub address: Box<Option<Address>>,

    /// Owner's email address.
    pub email: Box<Option<String>>,

    /// Owner's full name.
    pub name: Box<Option<String>>,

    /// Owner's phone number (including extension).
    pub phone: Box<Option<String>>,

    /// Verified owner's address.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_address: Box<Option<Address>>,

    /// Verified owner's email address.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_email: Box<Option<String>>,

    /// Verified owner's full name.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Box<Option<String>>,

    /// Verified owner's phone number (including extension).
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_phone: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceReceiverFlow {
    /// The address of the receiver source.
    ///
    /// This is the value that should be communicated to the customer to send their funds to.
    pub address: Box<Option<String>>,

    /// The total amount that was moved to your balance.
    ///
    /// This is almost always equal to the amount charged.
    /// In rare cases when customers deposit excess funds and we are unable to refund those, those funds get moved to your balance and show up in amount_charged as well.
    /// The amount charged is expressed in the source's currency.
    pub amount_charged: i64,

    /// The total amount received by the receiver source.
    ///
    /// `amount_received = amount_returned + amount_charged` should be true for consumed sources unless customers deposit excess funds.
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
    pub account_number: Box<Option<String>>,

    pub bank_name: Box<Option<String>>,

    pub fingerprint: Box<Option<String>>,

    pub refund_account_holder_name: Box<Option<String>>,

    pub refund_account_holder_type: Box<Option<String>>,

    pub refund_routing_number: Box<Option<String>>,

    pub routing_number: Box<Option<String>>,

    pub swift_code: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAchDebit {
    pub bank_name: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub fingerprint: Box<Option<String>>,

    pub last4: Box<Option<String>>,

    pub routing_number: Box<Option<String>>,

    #[serde(rename = "type")]
    pub type_: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAcssDebit {
    pub bank_address_city: Box<Option<String>>,

    pub bank_address_line_1: Box<Option<String>>,

    pub bank_address_line_2: Box<Option<String>>,

    pub bank_address_postal_code: Box<Option<String>>,

    pub bank_name: Box<Option<String>>,

    pub category: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub fingerprint: Box<Option<String>>,

    pub last4: Box<Option<String>>,

    pub routing_number: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAlipay {
    pub data_string: Box<Option<String>>,

    pub native_url: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeAuBecsDebit {
    pub bsb_number: Box<Option<String>>,

    pub fingerprint: Box<Option<String>>,

    pub last4: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeBancontact {
    pub bank_code: Box<Option<String>>,

    pub bank_name: Box<Option<String>>,

    pub bic: Box<Option<String>>,

    pub iban_last4: Box<Option<String>>,

    pub preferred_language: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeCard {
    pub address_line1_check: Box<Option<String>>,

    pub address_zip_check: Box<Option<String>>,

    pub brand: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub cvc_check: Box<Option<String>>,

    pub dynamic_last4: Box<Option<String>>,

    pub exp_month: Box<Option<i64>>,

    pub exp_year: Box<Option<i64>>,

    pub fingerprint: Box<Option<String>>,

    pub funding: Box<Option<String>>,

    pub last4: Box<Option<String>>,

    pub name: Box<Option<String>>,

    pub three_d_secure: Box<Option<String>>,

    pub tokenization_method: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeCardPresent {
    pub application_cryptogram: Box<Option<String>>,

    pub application_preferred_name: Box<Option<String>>,

    pub authorization_code: Box<Option<String>>,

    pub authorization_response_code: Box<Option<String>>,

    pub brand: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub cvm_type: Box<Option<String>>,

    pub data_type: Box<Option<String>>,

    pub dedicated_file_name: Box<Option<String>>,

    pub emv_auth_data: Box<Option<String>>,

    pub evidence_customer_signature: Box<Option<String>>,

    pub evidence_transaction_certificate: Box<Option<String>>,

    pub exp_month: Box<Option<i64>>,

    pub exp_year: Box<Option<i64>>,

    pub fingerprint: Box<Option<String>>,

    pub funding: Box<Option<String>>,

    pub last4: Box<Option<String>>,

    pub pos_device_id: Box<Option<String>>,

    pub pos_entry_mode: Box<Option<String>>,

    pub read_method: Box<Option<String>>,

    pub reader: Box<Option<String>>,

    pub terminal_verification_results: Box<Option<String>>,

    pub transaction_status_information: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeEps {
    pub reference: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeGiropay {
    pub bank_code: Box<Option<String>>,

    pub bank_name: Box<Option<String>>,

    pub bic: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeIdeal {
    pub bank: Box<Option<String>>,

    pub bic: Box<Option<String>>,

    pub iban_last4: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeKlarna {
    pub background_image_url: Box<Option<String>>,

    pub client_token: Box<Option<String>>,

    pub first_name: Box<Option<String>>,

    pub last_name: Box<Option<String>>,

    pub locale: Box<Option<String>>,

    pub logo_url: Box<Option<String>>,

    pub page_title: Box<Option<String>>,

    pub pay_later_asset_urls_descriptive: Box<Option<String>>,

    pub pay_later_asset_urls_standard: Box<Option<String>>,

    pub pay_later_name: Box<Option<String>>,

    pub pay_later_redirect_url: Box<Option<String>>,

    pub pay_now_asset_urls_descriptive: Box<Option<String>>,

    pub pay_now_asset_urls_standard: Box<Option<String>>,

    pub pay_now_name: Box<Option<String>>,

    pub pay_now_redirect_url: Box<Option<String>>,

    pub pay_over_time_asset_urls_descriptive: Box<Option<String>>,

    pub pay_over_time_asset_urls_standard: Box<Option<String>>,

    pub pay_over_time_name: Box<Option<String>>,

    pub pay_over_time_redirect_url: Box<Option<String>>,

    pub payment_method_categories: Box<Option<String>>,

    pub purchase_country: Box<Option<String>>,

    pub purchase_type: Box<Option<String>>,

    pub redirect_url: Box<Option<String>>,

    pub shipping_delay: Box<Option<i64>>,

    pub shipping_first_name: Box<Option<String>>,

    pub shipping_last_name: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeMultibanco {
    pub entity: Box<Option<String>>,

    pub reference: Box<Option<String>>,

    pub refund_account_holder_address_city: Box<Option<String>>,

    pub refund_account_holder_address_country: Box<Option<String>>,

    pub refund_account_holder_address_line1: Box<Option<String>>,

    pub refund_account_holder_address_line2: Box<Option<String>>,

    pub refund_account_holder_address_postal_code: Box<Option<String>>,

    pub refund_account_holder_address_state: Box<Option<String>>,

    pub refund_account_holder_name: Box<Option<String>>,

    pub refund_iban: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeP24 {
    pub reference: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeSepaDebit {
    pub bank_code: Box<Option<String>>,

    pub branch_code: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub fingerprint: Box<Option<String>>,

    pub last4: Box<Option<String>>,

    pub mandate_reference: Box<Option<String>>,

    pub mandate_url: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeSofort {
    pub bank_code: Box<Option<String>>,

    pub bank_name: Box<Option<String>>,

    pub bic: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub iban_last4: Box<Option<String>>,

    pub preferred_language: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeThreeDSecure {
    pub address_line1_check: Box<Option<String>>,

    pub address_zip_check: Box<Option<String>>,

    pub authenticated: Box<Option<bool>>,

    pub brand: Box<Option<String>>,

    pub card: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub customer: Box<Option<String>>,

    pub cvc_check: Box<Option<String>>,

    pub dynamic_last4: Box<Option<String>>,

    pub exp_month: Box<Option<i64>>,

    pub exp_year: Box<Option<i64>>,

    pub fingerprint: Box<Option<String>>,

    pub funding: Box<Option<String>>,

    pub last4: Box<Option<String>>,

    pub name: Box<Option<String>>,

    pub three_d_secure: Box<Option<String>>,

    pub tokenization_method: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTypeWechat {
    pub prepay_id: Box<Option<String>>,

    pub qr_code_url: Box<Option<String>>,

    pub statement_descriptor: Box<Option<String>>,
}

/// The parameters for `Source::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSource<'a> {
    /// Amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    /// Not supported for `receiver` type sources, where charge amount may not be specified until funds land.
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
    pub mandate: Box<Option<SourceMandateParams>>,

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
    pub receiver: Box<Option<CreateSourceReceiver>>,

    /// Parameters required for the redirect flow.
    ///
    /// Required if the source is authenticated by a redirect (`flow` is `redirect`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Box<Option<CreateSourceRedirect>>,

    /// Information about the items and shipping associated with the source.
    ///
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Box<Option<CreateSourceSourceOrder>>,

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
    /// Required unless `customer` and `original_source` are specified (see the [Cloning card Sources](https://stripe.com/docs/sources/connect#cloning-card-sources) guide).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
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
            source_order: Default::default(),
            statement_descriptor: Default::default(),
            token: Default::default(),
            type_: Default::default(),
        }
    }
}

/// The parameters for `Source::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSources<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SourceId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SourceId>,
}

impl<'a> ListSources<'a> {
    pub fn new() -> Self {
        ListSources {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `Source::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSource<'a> {
    /// Amount associated with the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Box<Option<SourceMandateParams>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<BillingDetails>,

    /// Information about the items and shipping associated with the source.
    ///
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Box<Option<UpdateSourceSourceOrder>>,
}

impl<'a> UpdateSource<'a> {
    pub fn new() -> Self {
        UpdateSource {
            amount: Default::default(),
            expand: Default::default(),
            mandate: Default::default(),
            metadata: Default::default(),
            owner: Default::default(),
            source_order: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceReceiver {
    pub refund_attributes_method: Box<Option<SourceRefundNotificationMethod>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceRedirect {
    pub return_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceSourceOrder {
    pub items: Box<Option<Vec<CreateSourceSourceOrderItems>>>,

    pub shipping: Box<Option<CreateSourceSourceOrderShipping>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMandateParams {
    pub acceptance: Box<Option<SourceAcceptanceParams>>,

    pub amount: Box<Option<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    pub interval: Box<Option<SourceMandateInterval>>,

    pub notification_method: Box<Option<SourceMandateNotificationMethod>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSourceSourceOrder {
    pub items: Box<Option<Vec<UpdateSourceSourceOrderItems>>>,

    pub shipping: Box<Option<UpdateSourceSourceOrderShipping>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceSourceOrderItems {
    pub amount: Box<Option<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    pub description: Box<Option<String>>,

    pub parent: Box<Option<String>>,

    pub quantity: Box<Option<u64>>,

    #[serde(rename = "type")]
    pub type_: Box<Option<CreateSourceSourceOrderItemsType>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceSourceOrderShipping {
    pub address: CreateSourceSourceOrderShippingAddress,

    pub carrier: Box<Option<String>>,

    pub name: Box<Option<String>>,

    pub phone: Box<Option<String>>,

    pub tracking_number: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceAcceptanceParams {
    pub date: Box<Option<Timestamp>>,

    pub ip: Box<Option<String>>,

    pub offline: Box<Option<SourceAcceptanceOfflineParams>>,

    pub online: Box<Option<SourceAcceptanceOnlineParams>>,

    pub status: SourceAcceptanceParamsStatus,

    #[serde(rename = "type")]
    pub type_: Box<Option<SourceAcceptanceParamsType>>,

    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSourceSourceOrderItems {
    pub amount: Box<Option<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    pub description: Box<Option<String>>,

    pub parent: Box<Option<String>>,

    pub quantity: Box<Option<u64>>,

    #[serde(rename = "type")]
    pub type_: Box<Option<UpdateSourceSourceOrderItemsType>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSourceSourceOrderShipping {
    pub address: UpdateSourceSourceOrderShippingAddress,

    pub carrier: Box<Option<String>>,

    pub name: Box<Option<String>>,

    pub phone: Box<Option<String>>,

    pub tracking_number: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSourceSourceOrderShippingAddress {
    pub city: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub line1: String,

    pub line2: Box<Option<String>>,

    pub postal_code: Box<Option<String>>,

    pub state: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceAcceptanceOfflineParams {
    pub contact_email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceAcceptanceOnlineParams {
    pub date: Box<Option<Timestamp>>,

    pub ip: Box<Option<String>>,

    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSourceSourceOrderShippingAddress {
    pub city: Box<Option<String>>,

    pub country: Box<Option<String>>,

    pub line1: String,

    pub line2: Box<Option<String>>,

    pub postal_code: Box<Option<String>>,

    pub state: Box<Option<String>>,
}

/// An enum representing the possible values of an `CreateSourceSourceOrderItems`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl CreateSourceSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSourceSourceOrderItemsType::Discount => "discount",
            CreateSourceSourceOrderItemsType::Shipping => "shipping",
            CreateSourceSourceOrderItemsType::Sku => "sku",
            CreateSourceSourceOrderItemsType::Tax => "tax",
        }
    }
}

impl AsRef<str> for CreateSourceSourceOrderItemsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
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
    pub fn as_str(self) -> &'static str {
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
    pub fn as_str(self) -> &'static str {
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

/// An enum representing the possible values of an `CreateSource`'s `flow` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
}

impl SourceFlow {
    pub fn as_str(self) -> &'static str {
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
    pub fn as_str(self) -> &'static str {
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
    pub fn as_str(self) -> &'static str {
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

/// An enum representing the possible values of an `CreateSourceReceiver`'s `refund_attributes_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceRefundNotificationMethod {
    Email,
    Manual,
    None,
}

impl SourceRefundNotificationMethod {
    pub fn as_str(self) -> &'static str {
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

/// An enum representing the possible values of an `Source`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Alipay,
    AuBecsDebit,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

impl SourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceType::AchCreditTransfer => "ach_credit_transfer",
            SourceType::AchDebit => "ach_debit",
            SourceType::AcssDebit => "acss_debit",
            SourceType::Alipay => "alipay",
            SourceType::AuBecsDebit => "au_becs_debit",
            SourceType::Bancontact => "bancontact",
            SourceType::Card => "card",
            SourceType::CardPresent => "card_present",
            SourceType::Eps => "eps",
            SourceType::Giropay => "giropay",
            SourceType::Ideal => "ideal",
            SourceType::Klarna => "klarna",
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

/// An enum representing the possible values of an `UpdateSourceSourceOrderItems`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl UpdateSourceSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSourceSourceOrderItemsType::Discount => "discount",
            UpdateSourceSourceOrderItemsType::Shipping => "shipping",
            UpdateSourceSourceOrderItemsType::Sku => "sku",
            UpdateSourceSourceOrderItemsType::Tax => "tax",
        }
    }
}

impl AsRef<str> for UpdateSourceSourceOrderItemsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
