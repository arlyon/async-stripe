/// You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow.
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryCreditReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryCreditReversalId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryCreditReversalNetwork,
    /// The ReceivedCredit being reversed.
    pub received_credit: String,
    /// Status of the CreditReversal
    pub status: stripe_treasury::TreasuryCreditReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedCreditsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
}
impl TreasuryCreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryCreditReversalNetwork::*;
        match self {
            Ach => "ach",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for TreasuryCreditReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryCreditReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryCreditReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryCreditReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryCreditReversalNetwork")
        })
    }
}
impl stripe_types::Object for TreasuryCreditReversal {
    type Id = stripe_treasury::TreasuryCreditReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryCreditReversalId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}
impl TreasuryCreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryCreditReversalStatus::*;
        match self {
            Canceled => "canceled",
            Posted => "posted",
            Processing => "processing",
        }
    }
}

impl std::str::FromStr for TreasuryCreditReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryCreditReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryCreditReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryCreditReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryCreditReversalStatus"))
    }
}
