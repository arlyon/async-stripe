// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::BalanceTransactionId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{
    ApplicationFee, ApplicationFeeRefund, Charge, ConnectCollectionTransfer, Currency, Dispute,
    IssuingAuthorization, IssuingTransaction, Payout, Refund, ReserveTransaction, Topup, Transfer,
    TransferReversal,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "BalanceTransaction".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BalanceTransaction {
    /// Unique identifier for the object.
    pub id: BalanceTransactionId,

    /// Gross amount of the transaction, in %s.
    pub amount: i64,

    /// The date the transaction's net funds will become available in the Stripe balance.
    pub available_on: Timestamp,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,

    /// Fees (in %s) paid for this transaction.
    pub fee: i64,

    /// Detailed breakdown of fees (in %s) paid for this transaction.
    pub fee_details: Vec<Fee>,

    /// Net amount of the transaction, in %s.
    pub net: i64,

    /// The Stripe object to which this transaction is related.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Expandable<BalanceTransactionSource>>,

    /// If the transaction's net funds are available in the Stripe balance yet.
    ///
    /// Either `available` or `pending`.
    pub status: BalanceTransactionStatus,

    /// Transaction type: `adjustment`, `advance`, `advance_funding`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    ///
    /// [Learn more](https://stripe.com/docs/reporting/balance-transaction-types) about balance transaction types and what they represent.
    #[serde(rename = "type")]
    pub type_: BalanceTransactionType,
}

impl Object for BalanceTransaction {
    type Id = BalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "balance_transaction"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fee {
    /// Amount of the fee, in cents.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
    #[serde(rename = "type")]
    pub type_: FeeType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum BalanceTransactionSource {
    ApplicationFee(ApplicationFee),
    Charge(Charge),
    ConnectCollectionTransfer(ConnectCollectionTransfer),
    Dispute(Dispute),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(ApplicationFeeRefund),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(IssuingAuthorization),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(IssuingTransaction),
    Payout(Payout),
    Refund(Refund),
    ReserveTransaction(ReserveTransaction),
    Topup(Topup),
    Transfer(Transfer),
    TransferReversal(TransferReversal),
}

/// An enum representing the possible values of an `BalanceTransaction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionStatus {
    Available,
    Pending,
}

/// An enum representing the possible values of an `BalanceTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionType {
    Adjustment,
    Advance,
    AdvanceFunding,
    ApplicationFee,
    ApplicationFeeRefund,
    Charge,
    ConnectCollectionTransfer,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    IssuingTransaction,
    Payment,
    PaymentFailureRefund,
    PaymentRefund,
    Payout,
    PayoutCancel,
    PayoutFailure,
    Refund,
    RefundFailure,
    ReserveTransaction,
    ReservedFunds,
    StripeFee,
    StripeFxFee,
    TaxFee,
    Topup,
    TopupReversal,
    Transfer,
    TransferCancel,
    TransferFailure,
    TransferRefund,
}

/// An enum representing the possible values of an `Fee`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    ApplicationFee,
    StripeFee,
    Tax,
}
