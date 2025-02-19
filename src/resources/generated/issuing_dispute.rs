// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingDisputeId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, File, IssuingDisputeStatus, IssuingTransaction,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingDispute".
///
/// For more details see <https://stripe.com/docs/api/issuing/disputes/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDispute {
    /// Unique identifier for the object.
    pub id: IssuingDisputeId,

    /// Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,

    /// List of balance transactions associated with the dispute.
    pub balance_transactions: Option<Vec<BalanceTransaction>>,

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

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<IssuingDisputeTreasury>,
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
    pub canceled: Option<IssuingDisputeCanceledEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<IssuingDisputeDuplicateEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<IssuingDisputeFraudulentEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<IssuingDisputeMerchandiseNotAsDescribedEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<IssuingDisputeNotReceivedEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<IssuingDisputeOtherEvidence>,

    /// The reason for filing the dispute.
    ///
    /// Its value will match the field containing the evidence.
    pub reason: IssuingDisputeEvidenceReason,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<IssuingDisputeServiceNotAsDescribedEvidence>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeCanceledEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// Date when order was canceled.
    pub canceled_at: Option<Timestamp>,

    /// Whether the cardholder was provided with a cancellation policy.
    pub cancellation_policy_provided: Option<bool>,

    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,

    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeCanceledEvidenceProductType>,

    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeCanceledEvidenceReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeDuplicateEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    pub card_statement: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    pub cash_receipt: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    pub check_image: Option<Expandable<File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    ///
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    pub original_transaction: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeFraudulentEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Date when the product was received.
    pub received_at: Option<Timestamp>,

    /// Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,

    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeNotReceivedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeNotReceivedEvidenceProductType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeOtherEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeOtherEvidenceProductType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Expandable<File>>,

    /// Date when order was canceled.
    pub canceled_at: Option<Timestamp>,

    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Date when the product was received.
    pub received_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeTreasury {
    /// The Treasury [DebitReversal](https://stripe.com/docs/api/treasury/debit_reversals) representing this Issuing dispute.
    pub debit_reversal: Option<String>,

    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
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
