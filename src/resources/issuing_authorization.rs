// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingAuthorizationId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, IssuingAuthorizationCheck, IssuingAuthorizationMethod,
    IssuingAuthorizationReason, IssuingCard, IssuingCardholder, IssuingTransaction, MerchantData,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorization".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorization {
    /// Unique identifier for the object.
    pub id: IssuingAuthorizationId,

    /// Whether the authorization has been approved.
    pub approved: bool,

    /// How the card details were provided.
    ///
    /// One of `keyed_in`, `swipe`, `chip`, `contactless`, or `online`.
    pub authorization_method: IssuingAuthorizationMethod,

    /// The amount that has been authorized.
    ///
    /// This will be `0` when the object is created, and increase after it has been approved.
    pub authorized_amount: i64,

    /// The currency that was presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub authorized_currency: Currency,

    pub balance_transactions: Vec<BalanceTransaction>,

    pub card: IssuingCard,

    /// The cardholder to whom this authorization belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<Expandable<IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The amount the authorization is expected to be in `held_currency`.
    ///
    /// When Stripe holds funds from you, this is the amount reserved for the authorization.
    /// This will be `0` when the object is created, and increase after it has been approved.
    /// For multi-currency transactions, `held_amount` can be used to determine the expected exchange rate.
    pub held_amount: i64,

    /// The currency of the [held amount](https://stripe.com/docs/api#issuing_authorization_object-held_amount).
    ///
    /// This will always be the card currency.
    pub held_currency: Currency,

    pub is_held_amount_controllable: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub merchant_data: MerchantData,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The amount the user is requesting to be authorized.
    ///
    /// This field will only be non-zero during an `issuing.authorization.request` webhook.
    pub pending_authorized_amount: i64,

    /// The additional amount Stripe will hold if the authorization is approved.
    ///
    /// This field will only be non-zero during an `issuing.authorization.request` webhook.
    pub pending_held_amount: i64,

    pub request_history: Vec<IssuingAuthorizationRequest>,

    /// One of `pending`, `reversed`, or `closed`.
    pub status: String,

    pub transactions: Vec<IssuingTransaction>,

    pub verification_data: IssuingAuthorizationVerificationData,

    /// What, if any, digital wallet was used for this authorization.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<IssuingAuthorizationWalletProvider>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorizationRequest {
    /// Whether this request was approved.
    pub approved: bool,

    /// The amount that was authorized at the time of this request.
    pub authorized_amount: i64,

    /// The currency that was presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub authorized_currency: Currency,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The amount Stripe held from your account to fund the authorization, if the request was approved.
    pub held_amount: i64,

    /// The currency of the [held amount](https://stripe.com/docs/api#issuing_authorization_object-held_amount).
    pub held_currency: Currency,

    /// One of `authorization_controls`, `card_active`, `card_inactive`, `insufficient_funds`, `account_compliance_disabled`, `account_inactive`, `suspected_fraud`, `webhook_approved`, `webhook_declined`, or `webhook_timeout`.
    pub reason: IssuingAuthorizationReason,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorizationVerificationData {
    /// One of `match`, `mismatch`, or `not_provided`.
    pub address_line1_check: IssuingAuthorizationCheck,

    /// One of `match`, `mismatch`, or `not_provided`.
    pub address_zip_check: IssuingAuthorizationCheck,

    /// One of `match`, `mismatch`, or `not_provided`.
    pub cvc_check: IssuingAuthorizationCheck,
}

/// An enum representing the possible values of an `IssuingAuthorization`'s `wallet_provider` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}
