// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryCreditReversalId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, TreasuryTransaction};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryReceivedCreditsResourceCreditReversal".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryCreditReversal {
    /// Unique identifier for the object.
    pub id: TreasuryCreditReversalId,

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
    pub financial_account: String,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The rails used to reverse the funds.
    pub network: TreasuryCreditReversalNetwork,

    /// The ReceivedCredit being reversed.
    pub received_credit: String,

    /// Status of the CreditReversal.
    pub status: TreasuryCreditReversalStatus,

    pub status_transitions: TreasuryReceivedCreditsResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Option<Expandable<TreasuryTransaction>>,
}

impl Object for TreasuryCreditReversal {
    type Id = TreasuryCreditReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.credit_reversal"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {

    /// Timestamp describing when the CreditReversal changed status to `posted`.
    pub posted_at: Option<Timestamp>,
}

/// An enum representing the possible values of an `TreasuryCreditReversal`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
}

impl TreasuryCreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryCreditReversalNetwork::Ach => "ach",
            TreasuryCreditReversalNetwork::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for TreasuryCreditReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryCreditReversalNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryCreditReversal`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

impl TreasuryCreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryCreditReversalStatus::Canceled => "canceled",
            TreasuryCreditReversalStatus::Posted => "posted",
            TreasuryCreditReversalStatus::Processing => "processing",
        }
    }
}

impl AsRef<str> for TreasuryCreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryCreditReversalStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
