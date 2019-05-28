// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingDisputeId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, File, IssuingTransaction};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingDispute".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDispute {
    /// Unique identifier for the object.
    pub id: IssuingDisputeId,

    /// Disputed amount.
    ///
    /// Usually the amount of the `disputed_transaction`, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The currency the `disputed_transaction` was made in.
    pub currency: Currency,

    /// The transaction being disputed.
    pub disputed_transaction: Expandable<IssuingTransaction>,

    pub evidence: IssuingDisputeEvidence,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub metadata: Metadata,

    /// Reason for this dispute.
    ///
    /// One of `other` or `fraudulent`.
    pub reason: IssuingDisputeReason,

    /// Current status of dispute.
    ///
    /// One of `unsubmitted`, `under_review`, `won`, or `lost`.
    pub status: IssuingDisputeStatus,
}

impl Object for IssuingDispute {
    type Id = IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDisputeEvidence {
    /// Evidence to support a fraudulent dispute.
    ///
    /// This will only be present if your dispute's `reason` is `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<IssuingDisputeFraudulentEvidence>,

    /// Evidence to support an uncategorized dispute.
    ///
    /// This will only be present if your dispute's `reason` is `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<IssuingDisputeOtherEvidence>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDisputeFraudulentEvidence {
    /// Brief freeform text explaining why you are disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_explanation: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional file evidence supporting your dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDisputeOtherEvidence {
    /// Brief freeform text explaining why you are disputing this transaction.
    pub dispute_explanation: String,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional file evidence supporting your dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<Expandable<File>>,
}

/// An enum representing the possible values of an `IssuingDispute`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeReason {
    Fraudulent,
    Other,
}

/// An enum representing the possible values of an `IssuingDispute`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeStatus {
    Lost,
    UnderReview,
    Unsubmitted,
    Won,
}
