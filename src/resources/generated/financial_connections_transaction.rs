// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{FinancialConnectionsTransactionId};
use crate::params::{Object, Timestamp};
use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceTransaction".
///
/// For more details see <https://stripe.com/docs/api/financial_connections/transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsTransaction {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsTransactionId,

    /// The ID of the Financial Connections Account this transaction belongs to.
    pub account: String,

    /// The amount of this transaction, in cents (or local equivalent).
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The description of this transaction.
    pub description: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The status of the transaction.
    pub status: FinancialConnectionsTransactionStatus,

    pub status_transitions: BankConnectionsResourceTransactionResourceStatusTransitions,

    /// Time at which the transaction was transacted.
    ///
    /// Measured in seconds since the Unix epoch.
    pub transacted_at: Timestamp,

    /// The token of the transaction refresh that last updated or created this transaction.
    pub transaction_refresh: String,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,
}

impl Object for FinancialConnectionsTransaction {
    type Id = FinancialConnectionsTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceTransactionResourceStatusTransitions {

    /// Time at which this transaction posted.
    ///
    /// Measured in seconds since the Unix epoch.
    pub posted_at: Option<Timestamp>,

    /// Time at which this transaction was voided.
    ///
    /// Measured in seconds since the Unix epoch.
    pub void_at: Option<Timestamp>,
}

/// An enum representing the possible values of an `FinancialConnectionsTransaction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsTransactionStatus {
    Pending,
    Posted,
    Void,
}

impl FinancialConnectionsTransactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancialConnectionsTransactionStatus::Pending => "pending",
            FinancialConnectionsTransactionStatus::Posted => "posted",
            FinancialConnectionsTransactionStatus::Void => "void",
        }
    }
}

impl AsRef<str> for FinancialConnectionsTransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FinancialConnectionsTransactionStatus {
    fn default() -> Self {
        Self::Pending
    }
}
