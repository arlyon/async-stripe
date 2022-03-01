// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{BalanceTransactionId, PayoutId, SourceId};
use crate::params::{Expand, Expandable, List, Object, RangeQuery, Timestamp};
use crate::resources::{
    ApplicationFee, ApplicationFeeRefund, BalanceTransactionStatus, Charge,
    ConnectCollectionTransfer, Currency, Dispute, FeeType, IssuingAuthorization, IssuingDispute,
    IssuingTransaction, Payout, PlatformTaxFee, Refund, ReserveTransaction, TaxDeductedAtSource,
    Topup, Transfer, TransferReversal,
};

/// The resource representing a Stripe "BalanceTransaction".
///
/// For more details see <https://stripe.com/docs/api/balance_transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    pub description: Option<Box<String>>,

    /// The exchange rate used, if applicable, for this transaction.
    ///
    /// Specifically, if money was converted from currency A to currency B, then the `amount` in currency A, times `exchange_rate`, would be the `amount` in currency B.
    /// For example, suppose you charged a customer 10.00 EUR.
    /// Then the PaymentIntent's `amount` would be `1000` and `currency` would be `eur`.
    /// Suppose this was converted into 12.34 USD in your Stripe account.
    /// Then the BalanceTransaction's `amount` would be `1234`, `currency` would be `usd`, and `exchange_rate` would be `1.234`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<Box<f64>>,

    /// Fees (in %s) paid for this transaction.
    pub fee: i64,

    /// Detailed breakdown of fees (in %s) paid for this transaction.
    pub fee_details: Vec<Fee>,

    /// Net amount of the transaction, in %s.
    pub net: i64,

    /// [Learn more](https://stripe.com/docs/reports/reporting-categories) about how reporting categories can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,

    /// The Stripe object to which this transaction is related.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Expandable<BalanceTransactionSourceUnion>>>,

    /// If the transaction's net funds are available in the Stripe balance yet.
    ///
    /// Either `available` or `pending`.
    pub status: BalanceTransactionStatus,

    /// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    ///
    /// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
    /// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
    #[serde(rename = "type")]
    pub type_: BalanceTransactionType,
}

impl BalanceTransaction {
    /// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
    ///
    /// The transactions are returned in sorted order, with the most recent transactions appearing first.  Note that this endpoint was previously called “Balance history” and used the path `/v1/balance/history`.
    pub fn list(
        client: &Client,
        params: ListBalanceTransactions<'_>,
    ) -> Response<List<BalanceTransaction>> {
        client.get_query("/balance_transactions", &params)
    }

    /// Retrieves the balance transaction with the given ID.
    ///
    /// Note that this endpoint previously used the path `/v1/balance/history/:id`.
    pub fn retrieve(
        client: &Client,
        id: &BalanceTransactionId,
        expand: &[&str],
    ) -> Response<BalanceTransaction> {
        client.get_query(&format!("/balance_transactions/{}", id), &Expand { expand })
    }
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Fee {
    /// Amount of the fee, in cents.
    pub amount: i64,

    /// ID of the Connect application that earned the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<String>>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    /// Type of the fee, one of: `application_fee`, `stripe_fee` or `tax`.
    #[serde(rename = "type")]
    pub type_: FeeType,
}

/// The parameters for `BalanceTransaction::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListBalanceTransactions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return transactions in a certain currency.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<BalanceTransactionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<PayoutId>,

    /// Only returns the original transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<BalanceTransactionId>,

    /// Only returns transactions of the given type.
    ///
    /// One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
}

impl<'a> ListBalanceTransactions<'a> {
    pub fn new() -> Self {
        ListBalanceTransactions {
            created: Default::default(),
            currency: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payout: Default::default(),
            source: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum BalanceTransactionSourceUnion {
    ApplicationFee(ApplicationFee),
    Charge(Charge),
    ConnectCollectionTransfer(ConnectCollectionTransfer),
    Dispute(Dispute),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(ApplicationFeeRefund),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(IssuingAuthorization),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(IssuingTransaction),
    Payout(Payout),
    PlatformTaxFee(PlatformTaxFee),
    Refund(Refund),
    ReserveTransaction(ReserveTransaction),
    TaxDeductedAtSource(TaxDeductedAtSource),
    Topup(Topup),
    Transfer(Transfer),
    TransferReversal(TransferReversal),
}
impl std::default::Default for BalanceTransactionSourceUnion {
    fn default() -> Self {
        Self::ApplicationFee(Default::default())
    }
}

/// An enum representing the possible values of an `BalanceTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionType {
    Adjustment,
    Advance,
    AdvanceFunding,
    AnticipationRepayment,
    ApplicationFee,
    ApplicationFeeRefund,
    Charge,
    ConnectCollectionTransfer,
    Contribution,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    IssuingDispute,
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

impl BalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            BalanceTransactionType::Adjustment => "adjustment",
            BalanceTransactionType::Advance => "advance",
            BalanceTransactionType::AdvanceFunding => "advance_funding",
            BalanceTransactionType::AnticipationRepayment => "anticipation_repayment",
            BalanceTransactionType::ApplicationFee => "application_fee",
            BalanceTransactionType::ApplicationFeeRefund => "application_fee_refund",
            BalanceTransactionType::Charge => "charge",
            BalanceTransactionType::ConnectCollectionTransfer => "connect_collection_transfer",
            BalanceTransactionType::Contribution => "contribution",
            BalanceTransactionType::IssuingAuthorizationHold => "issuing_authorization_hold",
            BalanceTransactionType::IssuingAuthorizationRelease => "issuing_authorization_release",
            BalanceTransactionType::IssuingDispute => "issuing_dispute",
            BalanceTransactionType::IssuingTransaction => "issuing_transaction",
            BalanceTransactionType::Payment => "payment",
            BalanceTransactionType::PaymentFailureRefund => "payment_failure_refund",
            BalanceTransactionType::PaymentRefund => "payment_refund",
            BalanceTransactionType::Payout => "payout",
            BalanceTransactionType::PayoutCancel => "payout_cancel",
            BalanceTransactionType::PayoutFailure => "payout_failure",
            BalanceTransactionType::Refund => "refund",
            BalanceTransactionType::RefundFailure => "refund_failure",
            BalanceTransactionType::ReserveTransaction => "reserve_transaction",
            BalanceTransactionType::ReservedFunds => "reserved_funds",
            BalanceTransactionType::StripeFee => "stripe_fee",
            BalanceTransactionType::StripeFxFee => "stripe_fx_fee",
            BalanceTransactionType::TaxFee => "tax_fee",
            BalanceTransactionType::Topup => "topup",
            BalanceTransactionType::TopupReversal => "topup_reversal",
            BalanceTransactionType::Transfer => "transfer",
            BalanceTransactionType::TransferCancel => "transfer_cancel",
            BalanceTransactionType::TransferFailure => "transfer_failure",
            BalanceTransactionType::TransferRefund => "transfer_refund",
        }
    }
}

impl AsRef<str> for BalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BalanceTransactionType {
    fn default() -> Self {
        Self::Adjustment
    }
}
