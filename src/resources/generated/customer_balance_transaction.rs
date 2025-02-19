// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::CustomerBalanceTransactionId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{CreditNote, Currency, Customer, Invoice};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CustomerBalanceTransaction".
///
/// For more details see <https://stripe.com/docs/api/customer_balance_transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceTransaction {
    /// Unique identifier for the object.
    pub id: CustomerBalanceTransactionId,

    /// The amount of the transaction.
    ///
    /// A negative value is a credit for the customer's balance, and a positive value is a debit to the customer's `balance`.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the credit note (if any) related to the transaction.
    pub credit_note: Option<Expandable<CreditNote>>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the customer the transaction belongs to.
    pub customer: Expandable<Customer>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The customer's `balance` after the transaction was applied.
    ///
    /// A negative value decreases the amount due on the customer's next invoice.
    /// A positive value increases the amount due on the customer's next invoice.
    pub ending_balance: i64,

    /// The ID of the invoice (if any) related to the transaction.
    pub invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, or `unapplied_from_invoice`.
    ///
    /// See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
    #[serde(rename = "type")]
    pub type_: CustomerBalanceTransactionType,
}

impl Object for CustomerBalanceTransaction {
    type Id = CustomerBalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "customer_balance_transaction"
    }
}

/// An enum representing the possible values of an `CustomerBalanceTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
        match self {
            CustomerBalanceTransactionType::Adjustment => "adjustment",
            CustomerBalanceTransactionType::AppliedToInvoice => "applied_to_invoice",
            CustomerBalanceTransactionType::CreditNote => "credit_note",
            CustomerBalanceTransactionType::Initial => "initial",
            CustomerBalanceTransactionType::InvoiceOverpaid => "invoice_overpaid",
            CustomerBalanceTransactionType::InvoiceTooLarge => "invoice_too_large",
            CustomerBalanceTransactionType::InvoiceTooSmall => "invoice_too_small",
            CustomerBalanceTransactionType::Migration => "migration",
            CustomerBalanceTransactionType::UnappliedFromInvoice => "unapplied_from_invoice",
            CustomerBalanceTransactionType::UnspentReceiverCredit => "unspent_receiver_credit",
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
impl std::default::Default for CustomerBalanceTransactionType {
    fn default() -> Self {
        Self::Adjustment
    }
}
