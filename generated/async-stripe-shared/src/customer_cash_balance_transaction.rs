/// Customers with certain payments enabled have a cash balance, representing funds that were paid
/// by the customer to a merchant, but have not yet been allocated to a payment.
/// Cash Balance Transactions.
/// represent when funds are moved into or out of this balance.
/// This includes funding by the customer, allocation.
/// to payments, and refunds to the customer.
///
/// For more details see <<https://stripe.com/docs/api/cash_balance_transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerCashBalanceTransaction {
pub adjusted_for_overdraft: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>,
pub applied_to_payment: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
        /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: stripe_types::Currency,
    /// The customer whose available cash balance changed as a result of this transaction.
pub customer: stripe_types::Expandable<stripe_shared::Customer>,
        /// The total available cash balance for the specified currency after this transaction was applied.
    /// Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
pub ending_balance: i64,
pub funded: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,
    /// Unique identifier for the object.
pub id: stripe_shared::CustomerCashBalanceTransactionId,
        /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
        /// The amount by which the cash balance changed, represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// A positive value represents funds being added to the cash balance, a negative value represents funds being removed from the cash balance.
pub net_amount: i64,
pub refunded_from_payment: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,
pub transferred_to_balance: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance>,
        /// The type of the cash balance transaction.
    /// New types may be added in future.
    /// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[cfg_attr(feature = "deserialize", serde(rename = "type"))]
pub type_: CustomerCashBalanceTransactionType,
pub unapplied_from_payment: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,

}
#[doc(hidden)]
pub struct CustomerCashBalanceTransactionBuilder {
    adjusted_for_overdraft: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>>,
applied_to_payment: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>>,
created: Option<stripe_types::Timestamp>,
currency: Option<stripe_types::Currency>,
customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
ending_balance: Option<i64>,
funded: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>>,
id: Option<stripe_shared::CustomerCashBalanceTransactionId>,
livemode: Option<bool>,
net_amount: Option<i64>,
refunded_from_payment: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>>,
transferred_to_balance: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance>>,
type_: Option<CustomerCashBalanceTransactionType>,
unapplied_from_payment: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>>,

}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerCashBalanceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerCashBalanceTransaction>,
        builder: CustomerCashBalanceTransactionBuilder,
    }

    impl Visitor for Place<CustomerCashBalanceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerCashBalanceTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerCashBalanceTransactionBuilder {
        type Out = CustomerCashBalanceTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adjusted_for_overdraft" => Deserialize::begin(&mut self.adjusted_for_overdraft),
                "applied_to_payment" => Deserialize::begin(&mut self.applied_to_payment),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "ending_balance" => Deserialize::begin(&mut self.ending_balance),
                "funded" => Deserialize::begin(&mut self.funded),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "net_amount" => Deserialize::begin(&mut self.net_amount),
                "refunded_from_payment" => Deserialize::begin(&mut self.refunded_from_payment),
                "transferred_to_balance" => Deserialize::begin(&mut self.transferred_to_balance),
                "type" => Deserialize::begin(&mut self.type_),
                "unapplied_from_payment" => Deserialize::begin(&mut self.unapplied_from_payment),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                adjusted_for_overdraft: Deserialize::default(),
                applied_to_payment: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                ending_balance: Deserialize::default(),
                funded: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                net_amount: Deserialize::default(),
                refunded_from_payment: Deserialize::default(),
                transferred_to_balance: Deserialize::default(),
                type_: Deserialize::default(),
                unapplied_from_payment: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(adjusted_for_overdraft),
                Some(applied_to_payment),
                Some(created),
                Some(currency),
                Some(customer),
                Some(ending_balance),
                Some(funded),
                Some(id),
                Some(livemode),
                Some(net_amount),
                Some(refunded_from_payment),
                Some(transferred_to_balance),
                Some(type_),
                Some(unapplied_from_payment),
            ) = (
                self.adjusted_for_overdraft.take(),
                self.applied_to_payment.take(),
                self.created,
                self.currency,
                self.customer.take(),
                self.ending_balance,
                self.funded.take(),
                self.id.take(),
                self.livemode,
                self.net_amount,
                self.refunded_from_payment.take(),
                self.transferred_to_balance.take(),
                self.type_,
                self.unapplied_from_payment.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                adjusted_for_overdraft,
                applied_to_payment,
                created,
                currency,
                customer,
                ending_balance,
                funded,
                id,
                livemode,
                net_amount,
                refunded_from_payment,
                transferred_to_balance,
                type_,
                unapplied_from_payment,
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

    impl ObjectDeser for CustomerCashBalanceTransaction {
        type Builder = CustomerCashBalanceTransactionBuilder;
    }

    impl FromValueOpt for CustomerCashBalanceTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerCashBalanceTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "adjusted_for_overdraft" => {
                        b.adjusted_for_overdraft = FromValueOpt::from_value(v)
                    }
                    "applied_to_payment" => b.applied_to_payment = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "ending_balance" => b.ending_balance = FromValueOpt::from_value(v),
                    "funded" => b.funded = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "net_amount" => b.net_amount = FromValueOpt::from_value(v),
                    "refunded_from_payment" => {
                        b.refunded_from_payment = FromValueOpt::from_value(v)
                    }
                    "transferred_to_balance" => {
                        b.transferred_to_balance = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unapplied_from_payment" => {
                        b.unapplied_from_payment = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerCashBalanceTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CustomerCashBalanceTransaction", 15)?;
        s.serialize_field("adjusted_for_overdraft", &self.adjusted_for_overdraft)?;
        s.serialize_field("applied_to_payment", &self.applied_to_payment)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("ending_balance", &self.ending_balance)?;
        s.serialize_field("funded", &self.funded)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("net_amount", &self.net_amount)?;
        s.serialize_field("refunded_from_payment", &self.refunded_from_payment)?;
        s.serialize_field("transferred_to_balance", &self.transferred_to_balance)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("unapplied_from_payment", &self.unapplied_from_payment)?;

        s.serialize_field("object", "customer_cash_balance_transaction")?;
        s.end()
    }
}
/// The type of the cash balance transaction.
/// New types may be added in future.
/// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerCashBalanceTransactionType {
    AdjustedForOverdraft,
    AppliedToPayment,
    Funded,
    FundingReversed,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    TransferredToBalance,
    UnappliedFromPayment,
}
impl CustomerCashBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use CustomerCashBalanceTransactionType::*;
        match self {
            AdjustedForOverdraft => "adjusted_for_overdraft",
            AppliedToPayment => "applied_to_payment",
            Funded => "funded",
            FundingReversed => "funding_reversed",
            RefundedFromPayment => "refunded_from_payment",
            ReturnCanceled => "return_canceled",
            ReturnInitiated => "return_initiated",
            TransferredToBalance => "transferred_to_balance",
            UnappliedFromPayment => "unapplied_from_payment",
        }
    }
}

impl std::str::FromStr for CustomerCashBalanceTransactionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerCashBalanceTransactionType::*;
        match s {
            "adjusted_for_overdraft" => Ok(AdjustedForOverdraft),
            "applied_to_payment" => Ok(AppliedToPayment),
            "funded" => Ok(Funded),
            "funding_reversed" => Ok(FundingReversed),
            "refunded_from_payment" => Ok(RefundedFromPayment),
            "return_canceled" => Ok(ReturnCanceled),
            "return_initiated" => Ok(ReturnInitiated),
            "transferred_to_balance" => Ok(TransferredToBalance),
            "unapplied_from_payment" => Ok(UnappliedFromPayment),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerCashBalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerCashBalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerCashBalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CustomerCashBalanceTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerCashBalanceTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerCashBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerCashBalanceTransactionType")
        })
    }
}
impl stripe_types::Object for CustomerCashBalanceTransaction {
    type Id = stripe_shared::CustomerCashBalanceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CustomerCashBalanceTransactionId);
