/// Customers with certain payments enabled have a cash balance, representing funds that were paid
/// by the customer to a merchant, but have not yet been allocated to a payment.
///
/// Cash Balance Transactions represent when funds are moved into or out of this balance.
/// This includes funding by the customer, allocation to payments, and refunds to the customer.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerCashBalanceTransaction {
#[serde(skip_serializing_if = "Option::is_none")]
pub applied_to_payment: Option<stripe_core::customer_cash_balance_transaction::applied_to_payment::AppliedToPayment>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: stripe_types::Currency,
    /// The customer whose available cash balance changed as a result of this transaction.
pub customer: stripe_types::Expandable<stripe_types::customer::Customer>,
    /// The total available cash balance for the specified currency after this transaction was applied.
    ///
    /// Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
pub ending_balance: i64,
#[serde(skip_serializing_if = "Option::is_none")]
pub funded: Option<stripe_core::customer_cash_balance_transaction::funded::Funded>,
    /// Unique identifier for the object.
pub id: stripe_core::customer_cash_balance_transaction::CustomerCashBalanceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// The amount by which the cash balance changed, represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// A positive value represents funds being added to the cash balance, a negative value represents funds being removed from the cash balance.
pub net_amount: i64,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: CustomerCashBalanceTransactionObject,
#[serde(skip_serializing_if = "Option::is_none")]
pub refunded_from_payment: Option<stripe_core::customer_cash_balance_transaction::refunded_from_payment::RefundedFromPayment>,
    /// The type of the cash balance transaction.
    ///
    /// One of `applied_to_payment`, `unapplied_from_payment`, `refunded_from_payment`, `funded`, `return_initiated`, or `return_canceled`.
    /// New types may be added in future.
    /// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[serde(rename = "type")]
pub type_: CustomerCashBalanceTransactionType,
#[serde(skip_serializing_if = "Option::is_none")]
pub unapplied_from_payment: Option<stripe_core::customer_cash_balance_transaction::unapplied_from_payment::UnappliedFromPayment>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerCashBalanceTransaction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerCashBalanceTransactionObject {
    CustomerCashBalanceTransaction,
}

impl CustomerCashBalanceTransactionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomerCashBalanceTransaction => "customer_cash_balance_transaction",
        }
    }
}

impl std::str::FromStr for CustomerCashBalanceTransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customer_cash_balance_transaction" => Ok(Self::CustomerCashBalanceTransaction),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerCashBalanceTransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerCashBalanceTransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerCashBalanceTransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerCashBalanceTransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerCashBalanceTransactionObject")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerCashBalanceTransactionObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CustomerCashBalanceTransactionObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CustomerCashBalanceTransactionObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The type of the cash balance transaction.
///
/// One of `applied_to_payment`, `unapplied_from_payment`, `refunded_from_payment`, `funded`, `return_initiated`, or `return_canceled`.
/// New types may be added in future.
/// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerCashBalanceTransactionType {
    AppliedToPayment,
    Funded,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    UnappliedFromPayment,
}

impl CustomerCashBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppliedToPayment => "applied_to_payment",
            Self::Funded => "funded",
            Self::RefundedFromPayment => "refunded_from_payment",
            Self::ReturnCanceled => "return_canceled",
            Self::ReturnInitiated => "return_initiated",
            Self::UnappliedFromPayment => "unapplied_from_payment",
        }
    }
}

impl std::str::FromStr for CustomerCashBalanceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "applied_to_payment" => Ok(Self::AppliedToPayment),
            "funded" => Ok(Self::Funded),
            "refunded_from_payment" => Ok(Self::RefundedFromPayment),
            "return_canceled" => Ok(Self::ReturnCanceled),
            "return_initiated" => Ok(Self::ReturnInitiated),
            "unapplied_from_payment" => Ok(Self::UnappliedFromPayment),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerCashBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerCashBalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerCashBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerCashBalanceTransactionType")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerCashBalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CustomerCashBalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CustomerCashBalanceTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for CustomerCashBalanceTransaction {
    type Id = stripe_core::customer_cash_balance_transaction::CustomerCashBalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CustomerCashBalanceTransactionId);
pub mod applied_to_payment;
pub use applied_to_payment::AppliedToPayment;
pub mod funded;
pub use funded::Funded;
pub mod refunded_from_payment;
pub use refunded_from_payment::RefundedFromPayment;
pub mod unapplied_from_payment;
pub use unapplied_from_payment::UnappliedFromPayment;
