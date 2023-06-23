/// Balance transactions represent funds moving through your Stripe account.
/// They're created for every type of transaction that comes into or flows out of your Stripe account balance.
///
/// Related guide: [Balance Transaction Types](https://stripe.com/docs/reports/balance-transaction-types).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BalanceTransaction {
    /// Gross amount of the transaction, in %s.
    pub amount: i64,
    /// The date the transaction's net funds will become available in the Stripe balance.
    pub available_on: stripe_types::Timestamp,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The exchange rate used, if applicable, for this transaction.
    ///
    /// Specifically, if money was converted from currency A to currency B, then the `amount` in currency A, times `exchange_rate`, would be the `amount` in currency B.
    /// For example, suppose you charged a customer 10.00 EUR.
    /// Then the PaymentIntent's `amount` would be `1000` and `currency` would be `eur`.
    /// Suppose this was converted into 12.34 USD in your Stripe account.
    /// Then the BalanceTransaction's `amount` would be `1234`, `currency` would be `usd`, and `exchange_rate` would be `1.234`.
    pub exchange_rate: Option<f64>,
    /// Fees (in %s) paid for this transaction.
    pub fee: i64,
    /// Detailed breakdown of fees (in %s) paid for this transaction.
    pub fee_details: Vec<stripe_core::balance_transaction::fee::Fee>,
    /// Unique identifier for the object.
    pub id: stripe_core::balance_transaction::BalanceTransactionId,
    /// Net amount of the transaction, in %s.
    pub net: i64,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: BalanceTransactionObject,
    /// [Learn more](https://stripe.com/docs/reports/reporting-categories) about how reporting categories can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,
    /// The Stripe object to which this transaction is related.
    pub source: Option<
        stripe_types::Expandable<stripe_core::balance_transaction_source::BalanceTransactionSource>,
    >,
    /// If the transaction's net funds are available in the Stripe balance yet.
    ///
    /// Either `available` or `pending`.
    pub status: String,
    /// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    ///
    /// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
    /// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
    #[serde(rename = "type")]
    pub type_: BalanceTransactionType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BalanceTransaction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionObject {
    BalanceTransaction,
}

impl BalanceTransactionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BalanceTransaction => "balance_transaction",
        }
    }
}

impl AsRef<str> for BalanceTransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BalanceTransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
///
/// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
/// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
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
            Self::Adjustment => "adjustment",
            Self::Advance => "advance",
            Self::AdvanceFunding => "advance_funding",
            Self::AnticipationRepayment => "anticipation_repayment",
            Self::ApplicationFee => "application_fee",
            Self::ApplicationFeeRefund => "application_fee_refund",
            Self::Charge => "charge",
            Self::ConnectCollectionTransfer => "connect_collection_transfer",
            Self::Contribution => "contribution",
            Self::IssuingAuthorizationHold => "issuing_authorization_hold",
            Self::IssuingAuthorizationRelease => "issuing_authorization_release",
            Self::IssuingDispute => "issuing_dispute",
            Self::IssuingTransaction => "issuing_transaction",
            Self::Payment => "payment",
            Self::PaymentFailureRefund => "payment_failure_refund",
            Self::PaymentRefund => "payment_refund",
            Self::Payout => "payout",
            Self::PayoutCancel => "payout_cancel",
            Self::PayoutFailure => "payout_failure",
            Self::Refund => "refund",
            Self::RefundFailure => "refund_failure",
            Self::ReserveTransaction => "reserve_transaction",
            Self::ReservedFunds => "reserved_funds",
            Self::StripeFee => "stripe_fee",
            Self::StripeFxFee => "stripe_fx_fee",
            Self::TaxFee => "tax_fee",
            Self::Topup => "topup",
            Self::TopupReversal => "topup_reversal",
            Self::Transfer => "transfer",
            Self::TransferCancel => "transfer_cancel",
            Self::TransferFailure => "transfer_failure",
            Self::TransferRefund => "transfer_refund",
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
impl stripe_types::Object for BalanceTransaction {
    type Id = stripe_core::balance_transaction::BalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(BalanceTransactionId, "txn_");
pub mod fee;
pub mod requests;
pub use fee::Fee;
