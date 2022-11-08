// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryDebitReversalId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, TreasuryTransaction};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryReceivedDebitsResourceDebitReversal".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryDebitReversal {
    /// Unique identifier for the object.
    pub id: TreasuryDebitReversalId,

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

    /// The FinancialAccount to reverse funds from.
    pub financial_account: Option<String>,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Other flows linked to a DebitReversal.
    pub linked_flows: Option<TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The rails used to reverse the funds.
    pub network: TreasuryDebitReversalNetwork,

    /// The ReceivedDebit being reversed.
    pub received_debit: String,

    /// Status of the DebitReversal.
    pub status: TreasuryDebitReversalStatus,

    pub status_transitions: TreasuryReceivedDebitsResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Option<Expandable<TreasuryTransaction>>,
}

impl Object for TreasuryDebitReversal {
    type Id = TreasuryDebitReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.debit_reversal"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {

    /// Set if there is an Issuing dispute associated with the DebitReversal.
    pub issuing_dispute: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {

    /// Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<Timestamp>,
}

/// An enum representing the possible values of an `TreasuryDebitReversal`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
}

impl TreasuryDebitReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryDebitReversalNetwork::Ach => "ach",
            TreasuryDebitReversalNetwork::Card => "card",
        }
    }
}

impl AsRef<str> for TreasuryDebitReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryDebitReversalNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryDebitReversal`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}

impl TreasuryDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryDebitReversalStatus::Failed => "failed",
            TreasuryDebitReversalStatus::Processing => "processing",
            TreasuryDebitReversalStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryDebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryDebitReversalStatus {
    fn default() -> Self {
        Self::Failed
    }
}
