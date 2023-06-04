/// A dispute occurs when a customer questions your charge with their card issuer.
/// When this happens, you're given the opportunity to respond to the dispute with
/// evidence that shows that the charge is legitimate.
///
/// You can find more information about the dispute process in our [Disputes and Fraud](/docs/disputes) documentation.  Related guide: [Disputes and Fraud](https://stripe.com/docs/disputes).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Dispute {
    /// Disputed amount.
    ///
    /// Usually the amount of the charge, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,
    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<crate::balance_transaction::BalanceTransaction>,
    /// ID of the charge that was disputed.
    pub charge: crate::Expandable<crate::charge::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    pub evidence: crate::dispute::evidence::Evidence,
    pub evidence_details: crate::dispute::evidence_details::EvidenceDetails,
    /// Unique identifier for the object.
    pub id: crate::dispute::DisputeId,
    /// If true, it is still possible to refund the disputed payment.
    ///
    /// Once the payment has been fully refunded, no further funds will be withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// Network-dependent reason code for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DisputeObject,
    /// ID of the PaymentIntent that was disputed.
    pub payment_intent: Option<crate::Expandable<crate::payment_intent::PaymentIntent>>,
    /// Reason given by cardholder for dispute.
    ///
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Read more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,
    /// Current status of dispute.
    ///
    /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
    pub status: DisputeStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Dispute {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DisputeObject {
    Dispute,
}

impl DisputeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dispute => "dispute",
        }
    }
}

impl AsRef<str> for DisputeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Current status of dispute.
///
/// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    ChargeRefunded,
    Lost,
    NeedsResponse,
    UnderReview,
    WarningClosed,
    WarningNeedsResponse,
    WarningUnderReview,
    Won,
}

impl DisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeRefunded => "charge_refunded",
            Self::Lost => "lost",
            Self::NeedsResponse => "needs_response",
            Self::UnderReview => "under_review",
            Self::WarningClosed => "warning_closed",
            Self::WarningNeedsResponse => "warning_needs_response",
            Self::WarningUnderReview => "warning_under_review",
            Self::Won => "won",
        }
    }
}

impl AsRef<str> for DisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Dispute {
    type Id = crate::dispute::DisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(DisputeId, "dp_" | "du_");
pub mod evidence;
pub mod requests;
pub use evidence::Evidence;
pub mod evidence_details;
pub use evidence_details::EvidenceDetails;
