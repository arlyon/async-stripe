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
    pub fee_details: Vec<stripe_types::balance_transaction::fee::Fee>,
    /// Unique identifier for the object.
    pub id: stripe_types::balance_transaction::BalanceTransactionId,
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
        stripe_types::Expandable<
            stripe_types::balance_transaction_source::BalanceTransactionSource,
        >,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for BalanceTransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balance_transaction" => Ok(Self::BalanceTransaction),

            _ => Err(()),
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
impl serde::Serialize for BalanceTransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BalanceTransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BalanceTransactionObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BalanceTransactionObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<BalanceTransactionObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BalanceTransactionObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
///
/// [Learn more](https://stripe.com/docs/reports/balance-transaction-types) about balance transaction types and what they represent.
/// If you are looking to classify transactions for accounting purposes, you might want to consider `reporting_category` instead.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for BalanceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adjustment" => Ok(Self::Adjustment),
            "advance" => Ok(Self::Advance),
            "advance_funding" => Ok(Self::AdvanceFunding),
            "anticipation_repayment" => Ok(Self::AnticipationRepayment),
            "application_fee" => Ok(Self::ApplicationFee),
            "application_fee_refund" => Ok(Self::ApplicationFeeRefund),
            "charge" => Ok(Self::Charge),
            "connect_collection_transfer" => Ok(Self::ConnectCollectionTransfer),
            "contribution" => Ok(Self::Contribution),
            "issuing_authorization_hold" => Ok(Self::IssuingAuthorizationHold),
            "issuing_authorization_release" => Ok(Self::IssuingAuthorizationRelease),
            "issuing_dispute" => Ok(Self::IssuingDispute),
            "issuing_transaction" => Ok(Self::IssuingTransaction),
            "payment" => Ok(Self::Payment),
            "payment_failure_refund" => Ok(Self::PaymentFailureRefund),
            "payment_refund" => Ok(Self::PaymentRefund),
            "payout" => Ok(Self::Payout),
            "payout_cancel" => Ok(Self::PayoutCancel),
            "payout_failure" => Ok(Self::PayoutFailure),
            "refund" => Ok(Self::Refund),
            "refund_failure" => Ok(Self::RefundFailure),
            "reserve_transaction" => Ok(Self::ReserveTransaction),
            "reserved_funds" => Ok(Self::ReservedFunds),
            "stripe_fee" => Ok(Self::StripeFee),
            "stripe_fx_fee" => Ok(Self::StripeFxFee),
            "tax_fee" => Ok(Self::TaxFee),
            "topup" => Ok(Self::Topup),
            "topup_reversal" => Ok(Self::TopupReversal),
            "transfer" => Ok(Self::Transfer),
            "transfer_cancel" => Ok(Self::TransferCancel),
            "transfer_failure" => Ok(Self::TransferFailure),
            "transfer_refund" => Ok(Self::TransferRefund),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BalanceTransactionType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<BalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BalanceTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for BalanceTransaction {
    type Id = stripe_types::balance_transaction::BalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(BalanceTransactionId, "txn_");
pub mod fee;
pub use fee::Fee;
