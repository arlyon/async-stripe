// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{IssuingDisputeSettlementDetailId};
use crate::params::{Object, Timestamp};
use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingDisputeSettlementDetail".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeSettlementDetail {
    /// Unique identifier for the object.
    pub id: IssuingDisputeSettlementDetailId,

    /// Disputed amount in the card’s currency and in the smallest currency unit.
    ///
    /// Usually the amount of the transaction, but can differ (usually because of currency fluctuation).
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The currency the original transaction was made in.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the linked dispute.
    pub dispute: String,

    /// The type of event corresponding to this dispute settlement detail, representing the stage in the dispute network lifecycle.
    pub event_type: IssuingDisputeSettlementDetailEventType,

    /// The card used to make the original transaction.
    pub issued_card: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The card network for this dispute settlement detail.
    ///
    /// One of ["visa", "mastercard", "maestro"].
    pub network: IssuingDisputeSettlementDetailNetwork,

    /// The ID of the linked card network settlement.
    pub settlement: Option<String>,
}

impl Object for IssuingDisputeSettlementDetail {
    type Id = IssuingDisputeSettlementDetailId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute_settlement_detail"
    }
}

/// An enum representing the possible values of an `IssuingDisputeSettlementDetail`'s `event_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeSettlementDetailEventType {
    Filing,
    Loss,
    Representment,
    Win,
}

impl IssuingDisputeSettlementDetailEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeSettlementDetailEventType::Filing => "filing",
            IssuingDisputeSettlementDetailEventType::Loss => "loss",
            IssuingDisputeSettlementDetailEventType::Representment => "representment",
            IssuingDisputeSettlementDetailEventType::Win => "win",
        }
    }
}

impl AsRef<str> for IssuingDisputeSettlementDetailEventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeSettlementDetailEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeSettlementDetailEventType {
    fn default() -> Self {
        Self::Filing
    }
}

/// An enum representing the possible values of an `IssuingDisputeSettlementDetail`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeSettlementDetailNetwork {
    Maestro,
    Mastercard,
    Visa,
}

impl IssuingDisputeSettlementDetailNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeSettlementDetailNetwork::Maestro => "maestro",
            IssuingDisputeSettlementDetailNetwork::Mastercard => "mastercard",
            IssuingDisputeSettlementDetailNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for IssuingDisputeSettlementDetailNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeSettlementDetailNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeSettlementDetailNetwork {
    fn default() -> Self {
        Self::Maestro
    }
}
