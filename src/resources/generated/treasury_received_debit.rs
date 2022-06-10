// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TreasuryReceivedDebitId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{
    Currency, TreasuryTransaction,
    UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
};

/// The resource representing a Stripe "ReceivedDebitsResourceTreasuryReceivedDebit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedDebit {
    /// Unique identifier for the object.
    pub id: TreasuryReceivedDebitId,

    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,

    /// Reason for the failure.
    ///
    /// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<TreasuryReceivedDebitFailureCode>,

    /// The FinancialAccount that funds were pulled from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details:
        Option<UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,

    pub linked_flows: ReceivedDebitsResourceTreasuryLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The network used for the ReceivedDebit.
    pub network: TreasuryReceivedDebitNetwork,

    /// Status of the ReceivedDebit.
    ///
    /// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
    /// The failure reason can be found under the `failure_code`.
    pub status: TreasuryReceivedDebitStatus,

    /// The Transaction associated with this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Expandable<TreasuryTransaction>>,
}

impl Object for TreasuryReceivedDebit {
    type Id = TreasuryReceivedDebitId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.received_debit"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReceivedDebitsResourceTreasuryLinkedFlows {
    /// Set if the ReceivedDebit is associated with an InboundTransfer's return of funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<String>,

    /// Set if the ReceivedDebit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,

    /// Set if the ReceivedDebit is also viewable as an [Issuing Dispute](https://stripe.com/docs/api#issuing_disputes) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
}

/// An enum representing the possible values of an `TreasuryReceivedDebit`'s `failure_code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}

impl TreasuryReceivedDebitFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedDebitFailureCode::AccountClosed => "account_closed",
            TreasuryReceivedDebitFailureCode::AccountFrozen => "account_frozen",
            TreasuryReceivedDebitFailureCode::InsufficientFunds => "insufficient_funds",
            TreasuryReceivedDebitFailureCode::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedDebitFailureCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

/// An enum representing the possible values of an `TreasuryReceivedDebit`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}

impl TreasuryReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedDebitNetwork::Ach => "ach",
            TreasuryReceivedDebitNetwork::Card => "card",
            TreasuryReceivedDebitNetwork::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedDebitNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryReceivedDebit`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitStatus {
    Failed,
    Succeeded,
}

impl TreasuryReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedDebitStatus::Failed => "failed",
            TreasuryReceivedDebitStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedDebitStatus {
    fn default() -> Self {
        Self::Failed
    }
}
