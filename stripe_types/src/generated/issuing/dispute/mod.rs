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
    pub balance_transactions: Option<Vec<stripe_types::balance_transaction::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The currency the `transaction` was made in.
    pub currency: stripe_types::Currency,
    pub evidence: stripe_types::issuing::dispute::evidence::Evidence,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing::dispute::IssuingDisputeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DisputeObject,
    /// Current status of the dispute.
    pub status: DisputeStatus,
    /// The transaction being disputed.
    pub transaction: stripe_types::Expandable<stripe_types::issuing::transaction::Transaction>,
    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types::issuing::dispute::treasury::Treasury>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DisputeObject {
    IssuingDispute,
}

impl DisputeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IssuingDispute => "issuing.dispute",
        }
    }
}

impl std::str::FromStr for DisputeObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "issuing.dispute" => Ok(Self::IssuingDispute),

            _ => Err(()),
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
impl serde::Serialize for DisputeObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputeObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DisputeObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DisputeObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputeObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Current status of the dispute.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for DisputeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "expired" => Ok(Self::Expired),
            "lost" => Ok(Self::Lost),
            "submitted" => Ok(Self::Submitted),
            "unsubmitted" => Ok(Self::Unsubmitted),
            "won" => Ok(Self::Won),

            _ => Err(()),
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
impl serde::Serialize for DisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DisputeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DisputeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputeStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Dispute {
    type Id = stripe_types::issuing::dispute::IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingDisputeId, "idp_");
pub mod evidence;
pub use evidence::Evidence;
pub mod treasury;
pub use treasury::Treasury;
