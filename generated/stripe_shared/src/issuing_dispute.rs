/// As a [card issuer](https://stripe.com/docs/issuing), you can dispute transactions that the cardholder does not recognize, suspects to be fraudulent, or has other issues with.
///
/// Related guide: [Issuing disputes](https://stripe.com/docs/issuing/purchases/disputes)
///
/// For more details see <<https://stripe.com/docs/api/issuing/disputes/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingDispute {
    /// Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,
    /// List of balance transactions associated with the dispute.
    pub balance_transactions: Option<Vec<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The currency the `transaction` was made in.
    pub currency: stripe_types::Currency,
    pub evidence: stripe_shared::IssuingDisputeEvidence,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingDisputeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Current status of the dispute.
    pub status: stripe_shared::IssuingDisputeStatus,
    /// The transaction being disputed.
    pub transaction: stripe_types::Expandable<stripe_shared::IssuingTransaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_shared::IssuingDisputeTreasury>,
}
impl stripe_types::Object for IssuingDispute {
    type Id = stripe_shared::IssuingDisputeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(IssuingDisputeId, "idp_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingDisputeStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}
impl IssuingDisputeStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingDisputeStatus::*;
        match self {
            Expired => "expired",
            Lost => "lost",
            Submitted => "submitted",
            Unsubmitted => "unsubmitted",
            Won => "won",
        }
    }
}

impl std::str::FromStr for IssuingDisputeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingDisputeStatus::*;
        match s {
            "expired" => Ok(Expired),
            "lost" => Ok(Lost),
            "submitted" => Ok(Submitted),
            "unsubmitted" => Ok(Unsubmitted),
            "won" => Ok(Won),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingDisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingDisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingDisputeStatus"))
    }
}
