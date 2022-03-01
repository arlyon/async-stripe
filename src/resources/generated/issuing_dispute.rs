// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::IssuingDisputeId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, File, IssuingDisputeStatus, IssuingTransaction,
};

/// The resource representing a Stripe "IssuingDispute".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDispute {
    /// Unique identifier for the object.
    pub id: IssuingDisputeId,

    /// Disputed amount.
    ///
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,

    /// List of balance transactions associated with the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Box<Vec<BalanceTransaction>>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The currency the `transaction` was made in.
    pub currency: Currency,

    pub evidence: IssuingDisputeEvidence,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Current status of the dispute.
    pub status: IssuingDisputeStatus,

    /// The transaction being disputed.
    pub transaction: Expandable<IssuingTransaction>,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Box<IssuingDisputeCanceledEvidence>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<Box<IssuingDisputeDuplicateEvidence>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<Box<IssuingDisputeFraudulentEvidence>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<Box<IssuingDisputeMerchandiseNotAsDescribedEvidence>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<Box<IssuingDisputeNotReceivedEvidence>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Box<IssuingDisputeOtherEvidence>>,

    /// The reason for filing the dispute.
    ///
    /// Its value will match the field containing the evidence.
    pub reason: IssuingDisputeEvidenceReason,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<Box<IssuingDisputeServiceNotAsDescribedEvidence>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeCanceledEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Box<Timestamp>>,

    /// Whether the cardholder was provided with a cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<Box<bool>>,

    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<Box<String>>,

    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<Box<Timestamp>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<Box<String>>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<Box<IssuingDisputeCanceledEvidenceProductType>>,

    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<Box<IssuingDisputeCanceledEvidenceReturnStatus>>,

    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<Box<Timestamp>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeDuplicateEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<Box<Expandable<File>>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<Box<Expandable<File>>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<Box<Expandable<File>>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,

    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    ///
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeFraudulentEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,

    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<Box<Timestamp>>,

    /// Description of the cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<Box<String>>,

    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<Box<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>>,

    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<Box<Timestamp>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeNotReceivedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<Box<Timestamp>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<Box<String>>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<Box<IssuingDisputeNotReceivedEvidenceProductType>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeOtherEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<Box<String>>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<Box<IssuingDisputeOtherEvidenceProductType>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<Box<Expandable<File>>>,

    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Box<Timestamp>>,

    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<Box<String>>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Box<String>>,

    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<Box<Timestamp>>,
}

/// An enum representing the possible values of an `IssuingDisputeCanceledEvidence`'s `product_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeCanceledEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeCanceledEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeCanceledEvidenceProductType::Merchandise => "merchandise",
            IssuingDisputeCanceledEvidenceProductType::Service => "service",
        }
    }
}

impl AsRef<str> for IssuingDisputeCanceledEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeCanceledEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeCanceledEvidenceProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

/// An enum representing the possible values of an `IssuingDisputeCanceledEvidence`'s `return_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeCanceledEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}

impl IssuingDisputeCanceledEvidenceReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeCanceledEvidenceReturnStatus::MerchantRejected => "merchant_rejected",
            IssuingDisputeCanceledEvidenceReturnStatus::Successful => "successful",
        }
    }
}

impl AsRef<str> for IssuingDisputeCanceledEvidenceReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeCanceledEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeCanceledEvidenceReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}

/// An enum representing the possible values of an `IssuingDisputeEvidence`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

impl IssuingDisputeEvidenceReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeEvidenceReason::Canceled => "canceled",
            IssuingDisputeEvidenceReason::Duplicate => "duplicate",
            IssuingDisputeEvidenceReason::Fraudulent => "fraudulent",
            IssuingDisputeEvidenceReason::MerchandiseNotAsDescribed => {
                "merchandise_not_as_described"
            }
            IssuingDisputeEvidenceReason::NotReceived => "not_received",
            IssuingDisputeEvidenceReason::Other => "other",
            IssuingDisputeEvidenceReason::ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl AsRef<str> for IssuingDisputeEvidenceReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeEvidenceReason {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `IssuingDisputeMerchandiseNotAsDescribedEvidence`'s `return_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}

impl IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::MerchantRejected => {
                "merchant_rejected"
            }
            IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus::Successful => "successful",
        }
    }
}

impl AsRef<str> for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}

/// An enum representing the possible values of an `IssuingDisputeNotReceivedEvidence`'s `product_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeNotReceivedEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeNotReceivedEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeNotReceivedEvidenceProductType::Merchandise => "merchandise",
            IssuingDisputeNotReceivedEvidenceProductType::Service => "service",
        }
    }
}

impl AsRef<str> for IssuingDisputeNotReceivedEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeNotReceivedEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeNotReceivedEvidenceProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

/// An enum representing the possible values of an `IssuingDisputeOtherEvidence`'s `product_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeOtherEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeOtherEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingDisputeOtherEvidenceProductType::Merchandise => "merchandise",
            IssuingDisputeOtherEvidenceProductType::Service => "service",
        }
    }
}

impl AsRef<str> for IssuingDisputeOtherEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeOtherEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingDisputeOtherEvidenceProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}
