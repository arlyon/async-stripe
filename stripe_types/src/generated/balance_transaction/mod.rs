/// Balance transactions represent funds moving through your Stripe account.
/// They're created for every type of transaction that comes into or flows out of your Stripe account balance.
///
/// Related guide: [Balance transaction types](https://stripe.com/docs/reports/balance-transaction-types).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    pub fee_details: Vec<stripe_types::Fee>,
    /// Unique identifier for the object.
    pub id: stripe_types::balance_transaction::BalanceTransactionId,
    /// Net amount of the transaction, in %s.
    pub net: i64,
    /// [Learn more](https://stripe.com/docs/reports/reporting-categories) about how reporting categories can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,
    /// The Stripe object to which this transaction is related.
    pub source: Option<stripe_types::Expandable<stripe_types::BalanceTransactionSource>>,
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
/// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
///
/// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
/// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl BalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use BalanceTransactionType::*;
        match self {
            Adjustment => "adjustment",
            Advance => "advance",
            AdvanceFunding => "advance_funding",
            AnticipationRepayment => "anticipation_repayment",
            ApplicationFee => "application_fee",
            ApplicationFeeRefund => "application_fee_refund",
            Charge => "charge",
            ConnectCollectionTransfer => "connect_collection_transfer",
            Contribution => "contribution",
            IssuingAuthorizationHold => "issuing_authorization_hold",
            IssuingAuthorizationRelease => "issuing_authorization_release",
            IssuingDispute => "issuing_dispute",
            IssuingTransaction => "issuing_transaction",
            Payment => "payment",
            PaymentFailureRefund => "payment_failure_refund",
            PaymentRefund => "payment_refund",
            Payout => "payout",
            PayoutCancel => "payout_cancel",
            PayoutFailure => "payout_failure",
            Refund => "refund",
            RefundFailure => "refund_failure",
            ReserveTransaction => "reserve_transaction",
            ReservedFunds => "reserved_funds",
            StripeFee => "stripe_fee",
            StripeFxFee => "stripe_fx_fee",
            TaxFee => "tax_fee",
            Topup => "topup",
            TopupReversal => "topup_reversal",
            Transfer => "transfer",
            TransferCancel => "transfer_cancel",
            TransferFailure => "transfer_failure",
            TransferRefund => "transfer_refund",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for BalanceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceTransactionType::*;
        match s {
            "adjustment" => Ok(Adjustment),
            "advance" => Ok(Advance),
            "advance_funding" => Ok(AdvanceFunding),
            "anticipation_repayment" => Ok(AnticipationRepayment),
            "application_fee" => Ok(ApplicationFee),
            "application_fee_refund" => Ok(ApplicationFeeRefund),
            "charge" => Ok(Charge),
            "connect_collection_transfer" => Ok(ConnectCollectionTransfer),
            "contribution" => Ok(Contribution),
            "issuing_authorization_hold" => Ok(IssuingAuthorizationHold),
            "issuing_authorization_release" => Ok(IssuingAuthorizationRelease),
            "issuing_dispute" => Ok(IssuingDispute),
            "issuing_transaction" => Ok(IssuingTransaction),
            "payment" => Ok(Payment),
            "payment_failure_refund" => Ok(PaymentFailureRefund),
            "payment_refund" => Ok(PaymentRefund),
            "payout" => Ok(Payout),
            "payout_cancel" => Ok(PayoutCancel),
            "payout_failure" => Ok(PayoutFailure),
            "refund" => Ok(Refund),
            "refund_failure" => Ok(RefundFailure),
            "reserve_transaction" => Ok(ReserveTransaction),
            "reserved_funds" => Ok(ReservedFunds),
            "stripe_fee" => Ok(StripeFee),
            "stripe_fx_fee" => Ok(StripeFxFee),
            "tax_fee" => Ok(TaxFee),
            "topup" => Ok(Topup),
            "topup_reversal" => Ok(TopupReversal),
            "transfer" => Ok(Transfer),
            "transfer_cancel" => Ok(TransferCancel),
            "transfer_failure" => Ok(TransferFailure),
            "transfer_refund" => Ok(TransferRefund),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(BalanceTransactionType::Unknown))
    }
}
impl stripe_types::Object for BalanceTransaction {
    type Id = stripe_types::balance_transaction::BalanceTransactionId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(BalanceTransactionId, "txn_");
