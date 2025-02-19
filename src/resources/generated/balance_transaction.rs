// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{BalanceTransactionId, PayoutId, SourceId};
use crate::params::{Expand, Expandable, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{
    BalanceTransactionSourceUnion, BalanceTransactionStatus, Currency, FeeType,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BalanceTransaction".
///
/// For more details see <https://stripe.com/docs/api/balance_transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceTransaction {
    /// Unique identifier for the object.
    pub id: BalanceTransactionId,

    /// Gross amount of this transaction (in cents (or local equivalent)).
    ///
    /// A positive value represents funds charged to another party, and a negative value represents funds sent to another party.
    pub amount: i64,

    /// The date that the transaction's net funds become available in the Stripe balance.
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
    pub description: Option<String>,

    /// If applicable, this transaction uses an exchange rate.
    ///
    /// If money converts from currency A to currency B, then the `amount` in currency A, multipled by the `exchange_rate`, equals the `amount` in currency B.
    /// For example, if you charge a customer 10.00 EUR, the PaymentIntent's `amount` is `1000` and `currency` is `eur`.
    /// If this converts to 12.34 USD in your Stripe account, the BalanceTransaction's `amount` is `1234`, its `currency` is `usd`, and the `exchange_rate` is `1.234`.
    pub exchange_rate: Option<f64>,

    /// Fees (in cents (or local equivalent)) paid for this transaction.
    ///
    /// Represented as a positive integer when assessed.
    pub fee: i64,

    /// Detailed breakdown of fees (in cents (or local equivalent)) paid for this transaction.
    pub fee_details: Vec<Fee>,

    /// Net impact to a Stripe balance (in cents (or local equivalent)).
    ///
    /// A positive value represents incrementing a Stripe balance, and a negative value decrementing a Stripe balance.
    /// You can calculate the net impact of a transaction on a balance by `amount` - `fee`.
    pub net: i64,

    /// Learn more about how [reporting categories](https://stripe.com/docs/reports/reporting-categories) can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,

    /// This transaction relates to the Stripe object.
    pub source: Option<Expandable<BalanceTransactionSourceUnion>>,

    /// The transaction's net funds status in the Stripe balance, which are either `available` or `pending`.
    pub status: BalanceTransactionStatus,

    /// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `climate_order_purchase`, `climate_order_refund`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_outbound`, `obligation_reversal_inbound`, `payment`, `payment_failure_refund`, `payment_network_reserve_hold`, `payment_network_reserve_release`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    ///
    /// Learn more about [balance transaction types and what they represent](https://stripe.com/docs/reports/balance-transaction-types).
    /// To classify transactions for accounting purposes, consider `reporting_category` instead.
    #[serde(rename = "type")]
    pub type_: BalanceTransactionType,
}

impl BalanceTransaction {
    /// Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth).
    ///
    /// The transactions are returned in sorted order, with the most recent transactions appearing first.  Note that this endpoint was previously called “Balance history” and used the path `/v1/balance/history`.
    pub fn list(
        client: &Client,
        params: &ListBalanceTransactions<'_>,
    ) -> Response<List<BalanceTransaction>> {
        client.get_query("/balance_transactions", params)
    }

    /// Retrieves the balance transaction with the given ID.
    ///
    /// Note that this endpoint previously used the path `/v1/balance/history/:id`.
    pub fn retrieve(
        client: &Client,
        id: &BalanceTransactionId,
        expand: &[&str],
    ) -> Response<BalanceTransaction> {
        client.get_query(&format!("/balance_transactions/{}", id), Expand { expand })
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
    pub application: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

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
    /// One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `climate_order_purchase`, `climate_order_refund`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_outbound`, `obligation_reversal_inbound`, `payment`, `payment_failure_refund`, `payment_network_reserve_hold`, `payment_network_reserve_release`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
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
impl Paginable for ListBalanceTransactions<'_> {
    type O = BalanceTransaction;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
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
    ClimateOrderPurchase,
    ClimateOrderRefund,
    ConnectCollectionTransfer,
    Contribution,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    IssuingDispute,
    IssuingTransaction,
    ObligationOutbound,
    ObligationReversalInbound,
    Payment,
    PaymentFailureRefund,
    PaymentNetworkReserveHold,
    PaymentNetworkReserveRelease,
    PaymentRefund,
    PaymentReversal,
    PaymentUnreconciled,
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
            BalanceTransactionType::ClimateOrderPurchase => "climate_order_purchase",
            BalanceTransactionType::ClimateOrderRefund => "climate_order_refund",
            BalanceTransactionType::ConnectCollectionTransfer => "connect_collection_transfer",
            BalanceTransactionType::Contribution => "contribution",
            BalanceTransactionType::IssuingAuthorizationHold => "issuing_authorization_hold",
            BalanceTransactionType::IssuingAuthorizationRelease => "issuing_authorization_release",
            BalanceTransactionType::IssuingDispute => "issuing_dispute",
            BalanceTransactionType::IssuingTransaction => "issuing_transaction",
            BalanceTransactionType::ObligationOutbound => "obligation_outbound",
            BalanceTransactionType::ObligationReversalInbound => "obligation_reversal_inbound",
            BalanceTransactionType::Payment => "payment",
            BalanceTransactionType::PaymentFailureRefund => "payment_failure_refund",
            BalanceTransactionType::PaymentNetworkReserveHold => "payment_network_reserve_hold",
            BalanceTransactionType::PaymentNetworkReserveRelease => {
                "payment_network_reserve_release"
            }
            BalanceTransactionType::PaymentRefund => "payment_refund",
            BalanceTransactionType::PaymentReversal => "payment_reversal",
            BalanceTransactionType::PaymentUnreconciled => "payment_unreconciled",
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
