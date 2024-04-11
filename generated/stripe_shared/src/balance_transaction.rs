/// Balance transactions represent funds moving through your Stripe account.
/// Stripe creates them for every type of transaction that enters or leaves your Stripe account balance.
///
/// Related guide: [Balance transaction types](https://stripe.com/docs/reports/balance-transaction-types).
///
/// For more details see <<https://stripe.com/docs/api/balance_transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: BalanceTransactionType,
}
#[doc(hidden)]
pub struct BalanceTransactionBuilder {
    amount: Option<i64>,
    available_on: Option<stripe_types::Timestamp>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    exchange_rate: Option<Option<f64>>,
    fee: Option<i64>,
    fee_details: Option<Vec<stripe_shared::Fee>>,
    id: Option<stripe_shared::BalanceTransactionId>,
    net: Option<i64>,
    reporting_category: Option<String>,
    source: Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransactionSource>>>,
    status: Option<String>,
    type_: Option<BalanceTransactionType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BalanceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceTransaction>,
        builder: BalanceTransactionBuilder,
    }

    impl Visitor for Place<BalanceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceTransactionBuilder {
        type Out = BalanceTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "available_on" => Deserialize::begin(&mut self.available_on),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "exchange_rate" => Deserialize::begin(&mut self.exchange_rate),
                "fee" => Deserialize::begin(&mut self.fee),
                "fee_details" => Deserialize::begin(&mut self.fee_details),
                "id" => Deserialize::begin(&mut self.id),
                "net" => Deserialize::begin(&mut self.net),
                "reporting_category" => Deserialize::begin(&mut self.reporting_category),
                "source" => Deserialize::begin(&mut self.source),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                available_on: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                exchange_rate: Deserialize::default(),
                fee: Deserialize::default(),
                fee_details: Deserialize::default(),
                id: Deserialize::default(),
                net: Deserialize::default(),
                reporting_category: Deserialize::default(),
                source: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                available_on: self.available_on?,
                created: self.created?,
                currency: self.currency?,
                description: self.description.take()?,
                exchange_rate: self.exchange_rate?,
                fee: self.fee?,
                fee_details: self.fee_details.take()?,
                id: self.id.take()?,
                net: self.net?,
                reporting_category: self.reporting_category.take()?,
                source: self.source.take()?,
                status: self.status.take()?,
                type_: self.type_?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BalanceTransaction {
        type Builder = BalanceTransactionBuilder;
    }

    impl FromValueOpt for BalanceTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "available_on" => b.available_on = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "exchange_rate" => b.exchange_rate = Some(FromValueOpt::from_value(v)?),
                    "fee" => b.fee = Some(FromValueOpt::from_value(v)?),
                    "fee_details" => b.fee_details = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "net" => b.net = Some(FromValueOpt::from_value(v)?),
                    "reporting_category" => {
                        b.reporting_category = Some(FromValueOpt::from_value(v)?)
                    }
                    "source" => b.source = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BalanceTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BalanceTransaction", 15)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("available_on", &self.available_on)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("exchange_rate", &self.exchange_rate)?;
        s.serialize_field("fee", &self.fee)?;
        s.serialize_field("fee_details", &self.fee_details)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("net", &self.net)?;
        s.serialize_field("reporting_category", &self.reporting_category)?;
        s.serialize_field("source", &self.source)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "balance_transaction")?;
        s.end()
    }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for BalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BalanceTransactionType::from_str(s).unwrap_or(BalanceTransactionType::Unknown));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BalanceTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
impl stripe_types::Object for BalanceTransaction {
    type Id = stripe_shared::BalanceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BalanceTransactionId);
