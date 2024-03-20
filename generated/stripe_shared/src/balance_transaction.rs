/// Balance transactions represent funds moving through your Stripe account.
/// Stripe creates them for every type of transaction that enters or leaves your Stripe account balance.
///
/// Related guide: [Balance transaction types](https://stripe.com/docs/reports/balance-transaction-types).
///
/// For more details see <<https://stripe.com/docs/api/balance_transactions/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceTransaction {
    /// Gross amount of this transaction (in cents (or local equivalent)).
    /// A positive value represents funds charged to another party, and a negative value represents funds sent to another party.
    pub amount: i64,
    /// The date that the transaction's net funds become available in the Stripe balance.
    pub available_on: stripe_types::Timestamp,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// If applicable, this transaction uses an exchange rate.
    /// If money converts from currency A to currency B, then the `amount` in currency A, multipled by the `exchange_rate`, equals the `amount` in currency B.
    /// For example, if you charge a customer 10.00 EUR, the PaymentIntent's `amount` is `1000` and `currency` is `eur`.
    /// If this converts to 12.34 USD in your Stripe account, the BalanceTransaction's `amount` is `1234`, its `currency` is `usd`, and the `exchange_rate` is `1.234`.
    pub exchange_rate: Option<f64>,
    /// Fees (in cents (or local equivalent)) paid for this transaction.
    /// Represented as a positive integer when assessed.
    pub fee: i64,
    /// Detailed breakdown of fees (in cents (or local equivalent)) paid for this transaction.
    pub fee_details: Vec<stripe_shared::Fee>,
    /// Unique identifier for the object.
    pub id: stripe_shared::BalanceTransactionId,
    /// Net impact to a Stripe balance (in cents (or local equivalent)).
    /// A positive value represents incrementing a Stripe balance, and a negative value decrementing a Stripe balance.
    /// You can calculate the net impact of a transaction on a balance by `amount` - `fee`.
    pub net: i64,
    /// Learn more about how [reporting categories](https://stripe.com/docs/reports/reporting-categories) can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,
    /// This transaction relates to the Stripe object.
    pub source: Option<stripe_types::Expandable<stripe_shared::BalanceTransactionSource>>,
    /// The transaction's net funds status in the Stripe balance, which are either `available` or `pending`.
    pub status: String,
    /// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `climate_order_purchase`, `climate_order_refund`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_outbound`, `obligation_reversal_inbound`, `payment`, `payment_failure_refund`, `payment_network_reserve_hold`, `payment_network_reserve_release`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    /// Learn more about [balance transaction types and what they represent](https://stripe.com/docs/reports/balance-transaction-types).
    /// To classify transactions for accounting purposes, consider `reporting_category` instead.
    #[serde(rename = "type")]
    pub type_: BalanceTransactionType,
}
/// Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `climate_order_purchase`, `climate_order_refund`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_outbound`, `obligation_reversal_inbound`, `payment`, `payment_failure_refund`, `payment_network_reserve_hold`, `payment_network_reserve_release`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
/// Learn more about [balance transaction types and what they represent](https://stripe.com/docs/reports/balance-transaction-types).
/// To classify transactions for accounting purposes, consider `reporting_category` instead.
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
            ClimateOrderPurchase => "climate_order_purchase",
            ClimateOrderRefund => "climate_order_refund",
            ConnectCollectionTransfer => "connect_collection_transfer",
            Contribution => "contribution",
            IssuingAuthorizationHold => "issuing_authorization_hold",
            IssuingAuthorizationRelease => "issuing_authorization_release",
            IssuingDispute => "issuing_dispute",
            IssuingTransaction => "issuing_transaction",
            ObligationOutbound => "obligation_outbound",
            ObligationReversalInbound => "obligation_reversal_inbound",
            Payment => "payment",
            PaymentFailureRefund => "payment_failure_refund",
            PaymentNetworkReserveHold => "payment_network_reserve_hold",
            PaymentNetworkReserveRelease => "payment_network_reserve_release",
            PaymentRefund => "payment_refund",
            PaymentReversal => "payment_reversal",
            PaymentUnreconciled => "payment_unreconciled",
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
            "climate_order_purchase" => Ok(ClimateOrderPurchase),
            "climate_order_refund" => Ok(ClimateOrderRefund),
            "connect_collection_transfer" => Ok(ConnectCollectionTransfer),
            "contribution" => Ok(Contribution),
            "issuing_authorization_hold" => Ok(IssuingAuthorizationHold),
            "issuing_authorization_release" => Ok(IssuingAuthorizationRelease),
            "issuing_dispute" => Ok(IssuingDispute),
            "issuing_transaction" => Ok(IssuingTransaction),
            "obligation_outbound" => Ok(ObligationOutbound),
            "obligation_reversal_inbound" => Ok(ObligationReversalInbound),
            "payment" => Ok(Payment),
            "payment_failure_refund" => Ok(PaymentFailureRefund),
            "payment_network_reserve_hold" => Ok(PaymentNetworkReserveHold),
            "payment_network_reserve_release" => Ok(PaymentNetworkReserveRelease),
            "payment_refund" => Ok(PaymentRefund),
            "payment_reversal" => Ok(PaymentReversal),
            "payment_unreconciled" => Ok(PaymentUnreconciled),
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
    type Id = stripe_shared::BalanceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(BalanceTransactionId, "txn_");
