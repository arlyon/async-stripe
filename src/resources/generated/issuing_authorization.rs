// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{IssuingAuthorizationId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{BalanceTransaction, Currency, IssuingAuthorizationAmountDetails, IssuingAuthorizationCheck, IssuingAuthorizationMethod, IssuingAuthorizationReason, IssuingCard, IssuingCardholder, IssuingTransaction, MerchantData};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorization".
///
/// For more details see <https://stripe.com/docs/api/issuing/authorizations/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorization {
    /// Unique identifier for the object.
    pub id: IssuingAuthorizationId,

    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<IssuingAuthorizationAmountDetails>,

    /// Whether the authorization has been approved.
    pub approved: bool,

    /// How the card details were provided.
    pub authorization_method: IssuingAuthorizationMethod,

    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<BalanceTransaction>,

    pub card: IssuingCard,

    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<Expandable<IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,

    /// The currency that was presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: Currency,

    pub merchant_data: MerchantData,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<IssuingAuthorizationNetworkData>,

    /// The pending authorization request.
    ///
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request: Option<IssuingAuthorizationPendingRequest>,

    /// History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g.
    ///
    /// based on your spending_controls).
    /// If the merchant changes the authorization by performing an incremental authorization, you can look at this field to see the previous requests for the authorization.
    /// This field can be helpful in determining why a given authorization was approved/declined.
    pub request_history: Vec<IssuingAuthorizationRequest>,

    /// The current status of the authorization in its lifecycle.
    pub status: IssuingAuthorizationStatus,

    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<IssuingTransaction>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<IssuingAuthorizationTreasury>,

    pub verification_data: IssuingAuthorizationVerificationData,

    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    pub wallet: Option<String>,
}

impl Object for IssuingAuthorization {
    type Id = IssuingAuthorizationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.authorization"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationNetworkData {

    /// Identifier assigned to the acquirer by the card network.
    ///
    /// Sometimes this value is not provided by the network; in this case, the value will be `null`.
    pub acquiring_institution_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationPendingRequest {

    /// The additional amount Stripe will hold if the authorization is approved, in the card's [currency](https://stripe.com/docs/api#issuing_authorization_object-pending-request-currency) and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<IssuingAuthorizationAmountDetails>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// If set `true`, you may provide [amount](https://stripe.com/docs/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    pub is_amount_controllable: bool,

    /// The amount the merchant is requesting to be authorized in the `merchant_currency`.
    ///
    /// The amount is in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,

    /// The local currency the merchant is requesting to authorize.
    pub merchant_currency: Currency,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationRequest {

    /// The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<IssuingAuthorizationAmountDetails>,

    /// Whether this request was approved.
    pub approved: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,

    /// The currency that was collected by the merchant and presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: Currency,

    /// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
    pub reason: IssuingAuthorizationReason,

    /// If approve/decline decision is directly responsed to the webhook with json payload and if the response is invalid (e.g., parsing errors), we surface the detailed message via this field.
    pub reason_message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationTreasury {

    /// The array of [ReceivedCredits](https://stripe.com/docs/api/treasury/received_credits) associated with this authorization.
    pub received_credits: Vec<String>,

    /// The array of [ReceivedDebits](https://stripe.com/docs/api/treasury/received_debits) associated with this authorization.
    pub received_debits: Vec<String>,

    /// The Treasury [Transaction](https://stripe.com/docs/api/treasury/transactions) associated with this authorization.
    pub transaction: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationVerificationData {

    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: IssuingAuthorizationCheck,

    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: IssuingAuthorizationCheck,

    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: IssuingAuthorizationCheck,

    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: IssuingAuthorizationCheck,
}

/// An enum representing the possible values of an `IssuingAuthorization`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}

impl IssuingAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingAuthorizationStatus::Closed => "closed",
            IssuingAuthorizationStatus::Pending => "pending",
            IssuingAuthorizationStatus::Reversed => "reversed",
        }
    }
}

impl AsRef<str> for IssuingAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingAuthorizationStatus {
    fn default() -> Self {
        Self::Closed
    }
}
