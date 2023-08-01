/// Each customer has a [Balance](https://stripe.com/docs/api/customers/object#customer_object-balance) value,
/// which denotes a debit or credit that's automatically applied to their next invoice upon finalization.
/// You may modify the value directly by using the [update customer API](https://stripe.com/docs/api/customers/update),
/// or by creating a Customer Balance Transaction, which increments or decrements the customer's `balance` by the specified `amount`.
///
/// Related guide: [Customer balance](https://stripe.com/docs/billing/customer/balance).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceTransaction {
    /// The amount of the transaction.
    ///
    /// A negative value is a credit for the customer's balance, and a positive value is a debit to the customer's `balance`.
    pub amount: i64,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the credit note (if any) related to the transaction.
    pub credit_note: Option<stripe_types::Expandable<stripe_types::credit_note::CreditNote>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the customer the transaction belongs to.
    pub customer: stripe_types::Expandable<stripe_types::customer::Customer>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The customer's `balance` after the transaction was applied.
    ///
    /// A negative value decreases the amount due on the customer's next invoice.
    /// A positive value increases the amount due on the customer's next invoice.
    pub ending_balance: i64,
    /// Unique identifier for the object.
    pub id: stripe_types::customer_balance_transaction::CustomerBalanceTransactionId,
    /// The ID of the invoice (if any) related to the transaction.
    pub invoice: Option<stripe_types::Expandable<stripe_types::invoice::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CustomerBalanceTransactionObject,
    /// Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, or `unapplied_from_invoice`.
    ///
    /// See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
    #[serde(rename = "type")]
    pub type_: CustomerBalanceTransactionType,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerBalanceTransactionObject {
    CustomerBalanceTransaction,
}

impl CustomerBalanceTransactionObject {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceTransactionObject::*;
        match self {
            CustomerBalanceTransaction => "customer_balance_transaction",
        }
    }
}

impl std::str::FromStr for CustomerBalanceTransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceTransactionObject::*;
        match s {
            "customer_balance_transaction" => Ok(CustomerBalanceTransaction),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerBalanceTransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceTransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerBalanceTransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceTransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerBalanceTransactionObject")
        })
    }
}
/// Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, or `unapplied_from_invoice`.
///
/// See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerBalanceTransactionType {
    Adjustment,
    AppliedToInvoice,
    CreditNote,
    Initial,
    InvoiceOverpaid,
    InvoiceTooLarge,
    InvoiceTooSmall,
    Migration,
    UnappliedFromInvoice,
    UnspentReceiverCredit,
}

impl CustomerBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceTransactionType::*;
        match self {
            Adjustment => "adjustment",
            AppliedToInvoice => "applied_to_invoice",
            CreditNote => "credit_note",
            Initial => "initial",
            InvoiceOverpaid => "invoice_overpaid",
            InvoiceTooLarge => "invoice_too_large",
            InvoiceTooSmall => "invoice_too_small",
            Migration => "migration",
            UnappliedFromInvoice => "unapplied_from_invoice",
            UnspentReceiverCredit => "unspent_receiver_credit",
        }
    }
}

impl std::str::FromStr for CustomerBalanceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceTransactionType::*;
        match s {
            "adjustment" => Ok(Adjustment),
            "applied_to_invoice" => Ok(AppliedToInvoice),
            "credit_note" => Ok(CreditNote),
            "initial" => Ok(Initial),
            "invoice_overpaid" => Ok(InvoiceOverpaid),
            "invoice_too_large" => Ok(InvoiceTooLarge),
            "invoice_too_small" => Ok(InvoiceTooSmall),
            "migration" => Ok(Migration),
            "unapplied_from_invoice" => Ok(UnappliedFromInvoice),
            "unspent_receiver_credit" => Ok(UnspentReceiverCredit),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerBalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerBalanceTransactionType")
        })
    }
}
impl stripe_types::Object for CustomerBalanceTransaction {
    type Id = stripe_types::customer_balance_transaction::CustomerBalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CustomerBalanceTransactionId, "cbtxn_");
