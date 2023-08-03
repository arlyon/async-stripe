/// Customers with certain payments enabled have a cash balance, representing funds that were paid
/// by the customer to a merchant, but have not yet been allocated to a payment.
///
/// Cash Balance Transactions represent when funds are moved into or out of this balance.
/// This includes funding by the customer, allocation to payments, and refunds to the customer.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerCashBalanceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_to_payment: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The customer whose available cash balance changed as a result of this transaction.
    pub customer: stripe_types::Expandable<stripe_types::Customer>,
    /// The total available cash balance for the specified currency after this transaction was applied.
    ///
    /// Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub ending_balance: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funded: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,
    /// Unique identifier for the object.
    pub id: stripe_types::customer_cash_balance_transaction::CustomerCashBalanceTransactionId,
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
    pub refunded_from_payment: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,
    /// The type of the cash balance transaction.
    ///
    /// New types may be added in future.
    /// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
    #[serde(rename = "type")]
    pub type_: CustomerCashBalanceTransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unapplied_from_payment: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerCashBalanceTransactionObject {
    CustomerCashBalanceTransaction,
}

impl CustomerCashBalanceTransactionObject {
    pub fn as_str(self) -> &'static str {
        use CustomerCashBalanceTransactionObject::*;
        match self {
            CustomerCashBalanceTransaction => "customer_cash_balance_transaction",
        }
    }
}

impl std::str::FromStr for CustomerCashBalanceTransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerCashBalanceTransactionObject::*;
        match s {
            "customer_cash_balance_transaction" => Ok(CustomerCashBalanceTransaction),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerCashBalanceTransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerCashBalanceTransactionObject"))
    }
}
/// The type of the cash balance transaction.
///
/// New types may be added in future.
/// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerCashBalanceTransactionType {
    AppliedToPayment,
    Funded,
    FundingReversed,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    UnappliedFromPayment,
}

impl CustomerCashBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use CustomerCashBalanceTransactionType::*;
        match self {
            AppliedToPayment => "applied_to_payment",
            Funded => "funded",
            FundingReversed => "funding_reversed",
            RefundedFromPayment => "refunded_from_payment",
            ReturnCanceled => "return_canceled",
            ReturnInitiated => "return_initiated",
            UnappliedFromPayment => "unapplied_from_payment",
        }
    }
}

impl std::str::FromStr for CustomerCashBalanceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerCashBalanceTransactionType::*;
        match s {
            "applied_to_payment" => Ok(AppliedToPayment),
            "funded" => Ok(Funded),
            "funding_reversed" => Ok(FundingReversed),
            "refunded_from_payment" => Ok(RefundedFromPayment),
            "return_canceled" => Ok(ReturnCanceled),
            "return_initiated" => Ok(ReturnInitiated),
            "unapplied_from_payment" => Ok(UnappliedFromPayment),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerCashBalanceTransactionType"))
    }
}
impl stripe_types::Object for CustomerCashBalanceTransaction {
    type Id = stripe_types::customer_cash_balance_transaction::CustomerCashBalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CustomerCashBalanceTransactionId);
