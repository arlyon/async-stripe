// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingTransactionId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, IssuingAuthorization, IssuingCard, IssuingCardholder,
    IssuingDispute, MerchantData,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingTransaction".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransaction {
    /// Unique identifier for the object.
    pub id: IssuingTransactionId,

    pub amount: i64,

    /// The `Authorization` object that led to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Expandable<IssuingAuthorization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// The card used to make this transaction.
    pub card: Expandable<IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<Expandable<IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<Expandable<IssuingDispute>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub merchant_data: MerchantData,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// One of `capture`, `refund`, `cash_withdrawal`, `refund_reversal`, `dispute`, or `dispute_loss`.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,
}

impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}

/// An enum representing the possible values of an `IssuingTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionType {
    Capture,
    CashWithdrawal,
    Dispute,
    DisputeLoss,
    Refund,
    RefundReversal,
}
