/// As a [card issuer](https://stripe.com/docs/issuing), you can dispute transactions that the cardholder does not recognize, suspects to be fraudulent, or has other issues with.
///
/// Related guide: [Disputing Transactions](https://stripe.com/docs/issuing/purchases/disputes).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Dispute {
    /// Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,
    /// List of balance transactions associated with the dispute.
    pub balance_transactions: Option<Vec<crate::balance_transaction::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// The currency the `transaction` was made in.
    pub currency: crate::Currency,
    pub evidence: crate::issuing::dispute::evidence::Evidence,
    /// Unique identifier for the object.
    pub id: crate::issuing::dispute::IssuingDisputeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DisputeObject,
    /// Current status of the dispute.
    pub status: DisputeStatus,
    /// The transaction being disputed.
    pub transaction: crate::Expandable<crate::issuing::transaction::Transaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<crate::issuing::dispute::treasury::Treasury>,
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
    #[serde(rename = "issuing.dispute")]
    IssuingDispute,
}

impl DisputeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IssuingDispute => "issuing.dispute",
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
/// Current status of the dispute.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}

impl DisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Submitted => "submitted",
            Self::Unsubmitted => "unsubmitted",
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
    type Id = crate::issuing::dispute::IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(IssuingDisputeId, "idp_");
pub mod evidence;
pub mod requests;
pub use evidence::Evidence;
pub mod treasury;
pub use treasury::Treasury;
